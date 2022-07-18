import json
import os
import shutil
from subprocess import Popen
import time
from typing import Tuple, Callable
import requests
import socket
from contextlib import closing
from pathlib import Path
import pytest


# Tracks processes that need to be killed at the end of the test
processes = []


@pytest.fixture(autouse=True)
def every_test():
    yield
    print()
    while len(processes) > 0:
        p = processes.pop(0)
        print(f"Killing {p.pid}")
        p.kill()


def get_port() -> int:
    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as s:
        s.bind(('', 0))
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        return s.getsockname()[1]


def get_env(p2p_port: int, grpc_port: int, http_port: int) -> dict[str, str]:
    env = os.environ.copy()
    env["solvio__CLUSTER__ENABLED"] = "true"
    env["solvio__CLUSTER__P2P__PORT"] = str(p2p_port)
    env["solvio__SERVICE__HTTP_PORT"] = str(http_port)
    env["solvio__SERVICE__GRPC_PORT"] = str(grpc_port)
    return env


def get_uri(port: int) -> str:
    return f"http://127.0.0.1:{port}"


def assert_http_ok(response: requests.Response):
    if response.status_code != 200:
        raise Exception(
            f"Http request failed with status {response.status_code} and contents: {response.json()}")


def assert_project_root():
    directory_path = os.getcwd()
    folder_name = os.path.basename(directory_path)
    assert folder_name == "solvio"


def get_solvio_exec() -> str:
    directory_path = os.getcwd()
    solvio_exec = directory_path + "/target/debug/solvio"
    return solvio_exec


# Starts a peer and returns its api_uri
def start_peer(peer_dir: Path, log_file: str, bootstrap_uri: str) -> str:
    p2p_port = get_port()
    grpc_port = get_port()
    http_port = get_port()
    env = get_env(p2p_port, grpc_port, http_port)
    log_file = open(log_file, "w")
    processes.append(
        Popen([get_solvio_exec(), "--bootstrap", bootstrap_uri], env=env, cwd=peer_dir, stderr=log_file))
    return get_uri(http_port)


# Starts a peer and returns its api_uri and p2p_uri
def start_first_peer(peer_dir: Path, log_file: str) -> Tuple[str, str]:
    p2p_port = get_port()
    grpc_port = get_port()
    http_port = get_port()
    env = get_env(p2p_port, grpc_port, http_port)
    log_file = open(log_file, "w")
    bootstrap_uri = get_uri(p2p_port)
    processes.append(
        Popen([get_solvio_exec(), "--uri", bootstrap_uri], env=env, cwd=peer_dir, stderr=log_file))
    return get_uri(http_port), bootstrap_uri


def make_peer_folders(base_path: Path, n_peers: int) -> list[Path]:
    peer_dirs = []
    for i in range(n_peers):
        peer_dir = base_path / f"peer{i}"
        peer_dir.mkdir()
        peer_dirs.append(peer_dir)
        shutil.copytree("config", peer_dir / "config")
    return peer_dirs


def get_cluster_info(peer_api_uri: str) -> dict:
    r = requests.get(f"{peer_api_uri}/cluster")
    assert_http_ok(r)
    res = r.json()["result"]
    return res


def print_clusters_info(peer_api_uris: [str]):
    for uri in peer_api_uris:
        print(json.dumps(get_cluster_info(uri), indent=4))


def get_collection_cluster_info(peer_api_uri: str, collection_name: str) -> dict:
    r = requests.get(f"{peer_api_uri}/collections/{collection_name}/cluster")
    assert_http_ok(r)
    res = r.json()["result"]
    return res


def print_collection_cluster_info(peer_api_uri: str, collection_name: str):
    print(json.dumps(get_collection_cluster_info(peer_api_uri, collection_name), indent=4))


def get_leader(peer_api_uri: str) -> str:
    r = requests.get(f"{peer_api_uri}/cluster")
    assert_http_ok(r)
    return r.json()["result"]["raft_info"]["leader"]


def check_leader(peer_api_uri: str, expected_leader: str) -> bool:
    try:
        r = requests.get(f"{peer_api_uri}/cluster")
        assert_http_ok(r)
        leader = r.json()["result"]["raft_info"]["leader"]
        correct_leader = leader == expected_leader
        if not correct_leader:
            print(f"Cluster leader invalid for peer {peer_api_uri} {leader}/{expected_leader}")
        return correct_leader
    except requests.exceptions.ConnectionError:
        # the api is not yet available - caller needs to retry
        print(f"Could not contact peer {peer_api_uri} to fetch cluster leader")
        return False


def leader_is_defined(peer_api_uri: str) -> bool:
    try:
        r = requests.get(f"{peer_api_uri}/cluster")
        assert_http_ok(r)
        leader = r.json()["result"]["raft_info"]["leader"]
        return leader is not None
    except requests.exceptions.ConnectionError:
        # the api is not yet available - caller needs to retry
        print(f"Could not contact peer {peer_api_uri} to fetch leader info")
        return False


def check_cluster_size(peer_api_uri: str, expected_size: int) -> bool:
    try:
        r = requests.get(f"{peer_api_uri}/cluster")
        assert_http_ok(r)
        peers = r.json()["result"]["peers"]
        correct_size = len(peers) == expected_size
        if not correct_size:
            print(f"Cluster size invalid for peer {peer_api_uri} {len(peers)}/{expected_size}")
        return correct_size
    except requests.exceptions.ConnectionError:
        # the api is not yet available - caller needs to retry
        print(f"Could not contact peer {peer_api_uri} to fetch cluster size")
        return False


def all_nodes_cluster_info_consistent(peer_api_uris: [str], expected_leader: str) -> bool:
    expected_size = len(peer_api_uris)
    for uri in peer_api_uris:
        if check_leader(uri, expected_leader) and check_cluster_size(uri, expected_size):
            continue
        else:
            return False
    return True


def collection_exists_on_all_peers(collection_name: str, peer_api_uris: [str]) -> bool:
    for uri in peer_api_uris:
        r = requests.get(f"{uri}/collections")
        assert_http_ok(r)
        collections = r.json()["result"]["collections"]
        filtered_collections = [c for c in collections if c['name'] == collection_name]
        if len(filtered_collections) == 0:
            print(f"Collection '{collection_name}' does not exist on peer {uri} found {json.dumps(collections, indent=4)}")
            return False
        else:
            continue
    return True


WAIT_TIME_SEC = 60
RETRY_INTERVAL_SEC = 0.5


def wait_peer_added(peer_api_uri: str, expected_size: int = 1) -> str:
    wait_for(check_cluster_size, peer_api_uri, expected_size)
    return get_leader(peer_api_uri)


def wait_for_uniform_cluster_status(peer_api_uris: [str], expected_leader: str):
    try:
        wait_for(all_nodes_cluster_info_consistent, peer_api_uris, expected_leader)
    except Exception as e:
        print_clusters_info(peer_api_uris)
        raise e


def wait_for_uniform_collection_existence(collection_name: str, peer_api_uris: [str]):
    try:
        wait_for(collection_exists_on_all_peers, collection_name, peer_api_uris)
    except Exception as e:
        print_clusters_info(peer_api_uris)
        raise e


def wait_for(condition: Callable[..., bool], *args):
    start = time.time()
    while not condition(*args):
        elapsed = time.time() - start
        if elapsed > WAIT_TIME_SEC:
            raise Exception(f"Timeout waiting for condition {condition.__name__} to be satisfied in {WAIT_TIME_SEC} seconds")
        else:
            time.sleep(RETRY_INTERVAL_SEC)
