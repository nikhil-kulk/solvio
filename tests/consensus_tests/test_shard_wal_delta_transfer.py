import multiprocessing
import pathlib
import random
import re
from time import sleep

from .fixtures import upsert_random_points, create_collection
from .utils import *

COLLECTION_NAME = "test_collection"


def update_points_in_loop(peer_url, collection_name, offset=0, throttle=False, duration=None):
    start = time.time()
    limit = 3

    while True:
        upsert_random_points(peer_url, limit, collection_name, offset=offset)
        offset += limit

        if throttle:
            sleep(random.uniform(0.01, 0.1))
        if duration is not None and (time.time() - start) > duration:
            break


def run_update_points_in_background(peer_url, collection_name, init_offset=0, throttle=False, duration=None):
    p = multiprocessing.Process(target=update_points_in_loop, args=(peer_url, collection_name, init_offset, throttle, duration))
    p.start()
    return p


# Test a WAL delta transfer between two that have no difference.
#
# No WAL records will be transferred as the WAL should be empty. We cannot
# assert that property in this test however. It is tested both ways.
#
# Test that data on the both sides is consistent
def test_empty_shard_wal_delta_transfer(capfd, tmp_path: pathlib.Path):
    assert_project_root()

    # seed port to reuse the same port for the restarted nodes
    peer_api_uris, peer_dirs, bootstrap_uri = start_cluster(tmp_path, 2, 20000)

    create_collection(peer_api_uris[0], shard_number=1, replication_factor=2)
    wait_collection_exists_and_active_on_all_peers(
        collection_name=COLLECTION_NAME,
        peer_api_uris=peer_api_uris
    )

    # Insert some initial number of points
    upsert_random_points(peer_api_uris[0], 100)

    cluster_info_0 = get_collection_cluster_info(peer_api_uris[0], COLLECTION_NAME)
    cluster_info_1 = get_collection_cluster_info(peer_api_uris[1], COLLECTION_NAME)

    shard_id = cluster_info_0['local_shards'][0]['shard_id']

    # Re-replicate shard from one node to another, should be no diff
    r = requests.post(
        f"{peer_api_uris[0]}/collections/{COLLECTION_NAME}/cluster", json={
            "replicate_shard": {
                "shard_id": shard_id,
                "from_peer_id": cluster_info_0['peer_id'],
                "to_peer_id": cluster_info_1['peer_id'],
                "method": "wal_delta",
            }
        })
    assert_http_ok(r)

    # Wait for end of shard transfer
    wait_for_collection_shard_transfers_count(peer_api_uris[0], COLLECTION_NAME, 0)

    # Confirm empty WAL delta based on debug message in stdout
    stdout, stderr = capfd.readouterr()
    assert "Resolved WAL delta that is empty" in stdout

    # Doing it the other way around should result in exactly the same
    r = requests.post(
        f"{peer_api_uris[1]}/collections/{COLLECTION_NAME}/cluster", json={
            "replicate_shard": {
                "shard_id": shard_id,
                "from_peer_id": cluster_info_1['peer_id'],
                "to_peer_id": cluster_info_0['peer_id'],
                "method": "wal_delta",
            }
        })
    assert_http_ok(r)

    # Wait for end of shard transfer
    wait_for_collection_shard_transfers_count(peer_api_uris[1], COLLECTION_NAME, 0)

    # Confirm empty WAL delta based on debug message in stdout
    stdout, stderr = capfd.readouterr()
    assert "Resolved WAL delta that is empty" in stdout

    cluster_info_0 = get_collection_cluster_info(peer_api_uris[0], COLLECTION_NAME)
    cluster_info_1 = get_collection_cluster_info(peer_api_uris[1], COLLECTION_NAME)
    assert len(cluster_info_0['local_shards']) == 1
    assert len(cluster_info_1['local_shards']) == 1

    # Match all points on all nodes exactly
    data = []
    for uri in peer_api_uris:
        r = requests.post(
            f"{uri}/collections/{COLLECTION_NAME}/points/scroll", json={
                "limit": 999999999,
                "with_vectors": True,
                "with_payload": True,
            }
        )
        assert_http_ok(r)
        data.append(r.json()["result"])
    assert data[0] == data[1]


# Test node recovery with a WAL delta transfer.
#
# The second node is killed while operations are ongoing. We later restart the
# node, and manually trigger rereplication to sync it up again.
#
# Test that data on the both sides is consistent
def test_shard_wal_delta_transfer_manual_recovery(tmp_path: pathlib.Path, capfd):
    assert_project_root()

    # Prevent automatic recovery on restarted node, so we can manually recover with a specific transfer method
    env={
        "solvio__STORAGE__PERFORMANCE__INCOMING_SHARD_TRANSFERS_LIMIT": "0",
        "solvio__STORAGE__PERFORMANCE__OUTGOING_SHARD_TRANSFERS_LIMIT": "0",
    }

    # seed port to reuse the same port for the restarted nodes
    peer_api_uris, peer_dirs, bootstrap_uri = start_cluster(tmp_path, 3, 20000, extra_env=env)

    create_collection(peer_api_uris[0], shard_number=1, replication_factor=3)
    wait_collection_exists_and_active_on_all_peers(
        collection_name=COLLECTION_NAME,
        peer_api_uris=peer_api_uris
    )

    transfer_collection_cluster_info = get_collection_cluster_info(peer_api_uris[0], COLLECTION_NAME)
    receiver_collection_cluster_info = get_collection_cluster_info(peer_api_uris[2], COLLECTION_NAME)

    from_peer_id = transfer_collection_cluster_info['peer_id']
    to_peer_id = receiver_collection_cluster_info['peer_id']

    shard_id = transfer_collection_cluster_info['local_shards'][0]['shard_id']

    # # Start pushing points to the cluster
    upload_process_1 = run_update_points_in_background(peer_api_uris[0], COLLECTION_NAME, init_offset=100000, throttle=True)
    upload_process_2 = run_update_points_in_background(peer_api_uris[1], COLLECTION_NAME, init_offset=200000, throttle=True)
    upload_process_3 = run_update_points_in_background(peer_api_uris[2], COLLECTION_NAME, init_offset=300000, throttle=True)

    sleep(3)

    # Kill last peer
    upload_process_3.kill()
    processes.pop().kill()

    sleep(5)

    # Restart the peer
    peer_api_uris[-1] = start_peer(peer_dirs[-1], "peer_2_restarted.log", bootstrap_uri, extra_env=env)
    wait_for_peer_online(peer_api_uris[-1], "/")

    # Recover shard with WAL delta transfer
    r = requests.post(
        f"{peer_api_uris[0]}/collections/{COLLECTION_NAME}/cluster", json={
            "replicate_shard": {
                "shard_id": shard_id,
                "from_peer_id": from_peer_id,
                "to_peer_id": to_peer_id,
                "method": "wal_delta",
            }
        })
    assert_http_ok(r)

    # Wait for end of shard transfer
    wait_for_collection_shard_transfers_count(peer_api_uris[0], COLLECTION_NAME, 0)

    # Confirm WAL delta transfer based on stdout logs, assert its size
    stdout, stderr = capfd.readouterr()
    delta_version, delta_size = re.search(r"Resolved WAL delta from (\d+), which counts (\d+) records", stdout).groups()
    assert int(delta_version) >= 80
    assert int(delta_size) >= 80

    receiver_collection_cluster_info = get_collection_cluster_info(peer_api_uris[2], COLLECTION_NAME)
    number_local_shards = len(receiver_collection_cluster_info['local_shards'])
    assert number_local_shards == 1

    upload_process_1.kill()
    upload_process_2.kill()
    sleep(1)

    # Match all points on all nodes exactly
    data = []
    for uri in peer_api_uris:
        r = requests.post(
            f"{uri}/collections/{COLLECTION_NAME}/points/scroll", json={
                "limit": 999999999,
                "with_vectors": True,
                "with_payload": True,
            }
        )
        assert_http_ok(r)
        data.append(r.json()["result"])
    assert data[0] == data[1] == data[2]


# Test the shard transfer fallback for WAL delta transfer.
#
# We replicate a shard with WAL delta transfer to a node that does not have any
# data for this shard yet. This is not supported in WAL delta transfer, so it
# should fall back to a different method.
#
# Test that data on the both sides is consistent
def test_shard_wal_delta_transfer_fallback(capfd, tmp_path: pathlib.Path):
    assert_project_root()

    # seed port to reuse the same port for the restarted nodes
    peer_api_uris, peer_dirs, bootstrap_uri = start_cluster(tmp_path, 3, 20000)

    create_collection(peer_api_uris[0], shard_number=3, replication_factor=1)
    wait_collection_exists_and_active_on_all_peers(
        collection_name=COLLECTION_NAME,
        peer_api_uris=peer_api_uris
    )

    # Insert some initial number of points
    upsert_random_points(peer_api_uris[0], 100)

    transfer_collection_cluster_info = get_collection_cluster_info(peer_api_uris[0], COLLECTION_NAME)
    receiver_collection_cluster_info = get_collection_cluster_info(peer_api_uris[2], COLLECTION_NAME)

    from_peer_id = transfer_collection_cluster_info['peer_id']
    to_peer_id = receiver_collection_cluster_info['peer_id']

    shard_id = transfer_collection_cluster_info['local_shards'][0]['shard_id']

    # Transfer shard from one node to another

    # Move shard `shard_id` to peer `target_peer_id`
    r = requests.post(
        f"{peer_api_uris[0]}/collections/{COLLECTION_NAME}/cluster", json={
            "replicate_shard": {
                "shard_id": shard_id,
                "from_peer_id": from_peer_id,
                "to_peer_id": to_peer_id,
                "method": "wal_delta",
            }
        })
    assert_http_ok(r)

    # Wait for end of shard transfer
    wait_for_collection_shard_transfers_count(peer_api_uris[0], COLLECTION_NAME, 0)

    # Confirm that we fall back to a different method after WAL delta fails
    stdout, stderr = capfd.readouterr()
    assert "Failed to do shard diff transfer, falling back to default method" in stdout

    receiver_collection_cluster_info = get_collection_cluster_info(peer_api_uris[2], COLLECTION_NAME)
    number_local_shards = len(receiver_collection_cluster_info['local_shards'])
    assert number_local_shards == 2

    # Match all points on all nodes exactly
    data = []
    for uri in peer_api_uris:
        r = requests.post(
            f"{uri}/collections/{COLLECTION_NAME}/points/scroll", json={
                "limit": 999999999,
                "with_vectors": True,
                "with_payload": True,
            }
        )
        assert_http_ok(r)
        data.append(r.json()["result"])
    assert data[0] == data[1] == data[2]


# If the difference between the two nodes is too big, the WAL delta transfer
# cannot be used. In this case, we should fall back to a different method - the stream transfer.
def test_shard_fallback_on_big_diff(tmp_path: pathlib.Path):
    assert_project_root()

    # Prevent automatic recovery on restarted node, so we can manually recover with a specific transfer method
    env={
        "solvio__STORAGE__PERFORMANCE__INCOMING_SHARD_TRANSFERS_LIMIT": "0",
        "solvio__STORAGE__PERFORMANCE__OUTGOING_SHARD_TRANSFERS_LIMIT": "0",
        "solvio__STORAGE__WAL__WAL_CAPACITY_MB": "1",
    }

    # seed port to reuse the same port for the restarted nodes
    peer_api_uris, peer_dirs, bootstrap_uri = start_cluster(tmp_path, 3, 20000, extra_env=env)

    create_collection(peer_api_uris[0], shard_number=1, replication_factor=3)
    wait_collection_exists_and_active_on_all_peers(
        collection_name=COLLECTION_NAME,
        peer_api_uris=peer_api_uris
    )

    transfer_collection_cluster_info = get_collection_cluster_info(peer_api_uris[0], COLLECTION_NAME)
    receiver_collection_cluster_info = get_collection_cluster_info(peer_api_uris[2], COLLECTION_NAME)

    from_peer_id = transfer_collection_cluster_info['peer_id']
    to_peer_id = receiver_collection_cluster_info['peer_id']

    shard_id = 0

    upsert_random_points(peer_api_uris[0], 100)

    sleep(1)

    # Kill last peer
    processes.pop().kill()

    sleep(1)

    for i in range(10):
        upsert_random_points(peer_api_uris[0], 1000, offset=100000 + i * 1000)

    sleep(1)

    # Restart the peer
    peer_api_uris[-1] = start_peer(peer_dirs[-1], "peer_2_restarted.log", bootstrap_uri, extra_env=env)
    wait_for_peer_online(peer_api_uris[-1], "/")


    # Move shard `shard_id` to peer `target_peer_id`
    r = requests.post(
        f"{peer_api_uris[0]}/collections/{COLLECTION_NAME}/cluster", json={
            "replicate_shard": {
                "shard_id": shard_id,
                "from_peer_id": from_peer_id,
                "to_peer_id": to_peer_id,
                "method": "wal_delta",
            }
        })
    assert_http_ok(r)

    # Wait for end of shard transfer
    wait_for_collection_shard_transfers_count(peer_api_uris[0], COLLECTION_NAME, 0)

    # Match all points on all nodes exactly
    data = []
    for uri in peer_api_uris:
        r = requests.post(
            f"{uri}/collections/{COLLECTION_NAME}/points/scroll", json={
                "limit": 999999999,
                "with_vectors": False,
                "with_payload": False,
            }
        )
        assert_http_ok(r)
        data.append(r.json()["result"])
    assert data[0] == data[1] == data[2]

