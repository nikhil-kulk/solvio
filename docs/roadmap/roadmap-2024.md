# Solvio 2024 Roadmap

Hi!
This document is our plan for Solvio development in 2024.
Previous year roadmap is available here:

* [Roadmap 2023](roadmap-2023.md)
* [Roadmap 2022](roadmap-2022.md)

Goals of the release:

* **Maintain easy upgrades** - we plan to keep backward compatibility for at least one minor version back (this stays the same in 2024).
  * That means that you can upgrade Solvio without any downtime and without any changes in your client code within one minor version.
  * Storage should be compatible between any two consequent versions, so you can upgrade Solvio with automatic data migration between consecutive versions.
* **Make serving easy on multi-billion scale** - Solvio already can serve billions of vectors cheaply, using techniques as quantization. In the 2024 year, we plan to make it even easier to scale it.
  * Faster and more reliable replications
  * Out-of-the-box read-write segregation
  * Specialized nodes and multi-region deployments
* **Better ecosystem** - in 2023 we introduced [fastembed](https://github.com/solvio/fastembed) to simplify embedding generation but keep it out of the core.
  In 2024 we plan to continue this trend: implement more advanced and specialized tools while keeping the core focused on the main use-case.
  * Advanced support for sparse vectors - we plan to make sparse vectors inference as fast and easy as the dense one.
  * Hybrid search out of the box with no overhead - something you can build with Solvio today, but in a more convenient way.
  * Practical RAG - battle-tested RAG practices with production-grade implementation.
* **Various similarity search scenarios** - develop vector similarity beyond just kNN search.

## How to contribute

If you are a Solvio user - Data Scientist, ML Engineer, or MLOps, the best contribution would be the feedback on your experience with Solvio.
Let us know whenever you have a problem, face an unexpected behavior, or see a lack of documentation.
You can do it in any convenient way - create an [issue](https://github.com/solvio/solvio/issues), start a [discussion](https://github.com/solvio/solvio/discussions), or drop up a [message](https://discord.gg/tdtYvXjC4h).
If you use Solvio or Metric Learning in your projects, we'd love to hear your story! Feel free to share articles and demos in our community.

For those familiar with Rust - check out our [contribution guide](../CONTRIBUTING.md).
If you have problems with code or architecture understanding - reach us at any time.
Feeling confident and want to contribute more? - Come to [work with us](https://solvio.join.com/)!

## Core Milestones

* 📃 Hybrid Search and Sparse Vectors
  * [x] Make Sparse Vectors serving as cheap and fast as Dense Vectors
  * [x] Introduce Hybrid Search into Solvio Client
    * [x] Dense + Sparse + Fusion in one request
    * [x] Customizable Re-Ranking

---

* 🏗️ Scalability
  * [x] Faster shard synchronization
    * [x] Non-blocking snapshotting
    * [x] Incremental replication
  * [ ] Specialized nodes
    * [ ] Read-only nodes
    * [ ] Indexing nodes
  * [ ] Multi-region deployments
    * [ ] Automatic replication over availability zones

---

* ⚙️ Performance
  * [ ] Specialized vector indexing for edge cases HNSW is not good at
  * [x] Text-index performance and resource consumption improvements
  * [x] IO optimizations for disk-bound workloads
  * [x] GPU acceleration

---

* 🏝️ New Data Exploration techniques
  * [x] Improvements in Discovery API to support more use-cases
  * [x] Multi-vectors support
  * [ ] Diversity Sampling
  * [ ] Better Aggregations
  * [ ] Advanced text filtering
    * [ ] Phrase queries
    * [ ] Logical operators
