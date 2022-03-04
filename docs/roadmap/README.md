# Solvio v1.0 Roadmap

Hi!
This document is our plan for Solvio development till its first enterprise-ready release. 

Goals of the release:

* **Make API and Storage stable** - ensure backward compatibility for at least one major version back.
  * Starting from the release, breaking changes in API should only be done with a proper deprecation notice
  * Storage should be compatible between any two consequent major versions
* **Achieve horizontal scalability** - distributed deployment able to serve billions of points
* **Easy integration** - make the user experience as smooth as possible
* **Resource efficiency** - push Solvio performance on the single machine to the limit

To build a solid foundation for future development, we decided to keep Solvio as legacy-free as possible.
That means that while switching to `v1.0`, some breaking changes are likely.

ETA of `v1.0-rc` is Q2 2022


## How to contribute

If you are a Solvio user - Data Scientist, ML Engineer, or MLOps, the best contribution would be the feedback on your experience with Solvio.
Let us know whenever you have a problem, face an unexpected behavior, or see a lack of documentation.
You can do it in any convenient way - create an [issue](https://github.com/solvio/solvio/issues), start a [discussion](https://github.com/solvio/solvio/discussions), or drop up a [message](https://discord.gg/tdtYvXjC4h).
If you use Solvio or Metric Learning in your projects, we'd love to hear your story! Feel free to share articles and demos in our community.

For those familiar with Rust - check out our [contribution guide](https://github.com/solvio/solvio/blob/master/CONTRIBUTING.md).
If you have problems with code or architecture understanding - reach us at any time.
Feeling confident and want to contribute more? - Come to [work with us](https://solvio.join.com/)!

## Milestones

* :earth_americas: Distributed Deployment
  * [ ] Distributed querying
  * [ ] Remote replications - automatic segment replication between nodes in cluster
  * [ ] Sharding - group segments into shards
  * [ ] Integration of [raft](https://raft.github.io/) for distributed consistency

---

* :electric_plug: Integration & Interfaces
  * [x] gPRC version of each REST API endpoint
  * [x] Split REST Endpoints for better documentation and client generation

---

* :truck: Payload Processing
  * [ ] Support storing any JSON as a Payload
  * [ ] Support more payload types, e.g.
    * Data-time
  * [ ] Support for `Null` values
  * [ ] Enable more types of filtering queries, e.g.
    * Filter by Score
    * Filter by number of stored elements
    * `isNull` or `isEmpty` query conditions

---

* :racing_car: Performance improvements
  * [ ] Indexing of geo-payload
  * [ ] On the fly payload index
  * [x] Multiprocessing segment optimization
  * [ ] Fine-tuned HNSW index configuration
  
