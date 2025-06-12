<p align="center">
  <img src="https://github.com/user-attachments/assets/b2f63e69-1c3c-4fb5-b236-1bb2529b1252" alt="Solvio">
</p>

<p align="center">
    <b>Vector Search Engine Powering Tomorrow’s AI Innovations</b>
</p>

**Solvio** functions as both a vector similarity search engine and a vector database, delivering a ready-to-use platform with an accessible API. This allows users to store, retrieve, and organize points—vectors paired with supplementary payload information. Solvio is engineered with robust filtering features, making it a versatile tool for applications like neural network-driven or semantic matching, faceted searches, and a variety of other uses. Through Solvio, embeddings or neural network encoders can be transformed into fully operational systems for tasks such as matching, exploring, suggesting, and far more.

## Getting Started

### Python

Kick off your Solvio journey in Python by installing the client library with:

```
pip install solvio-client
```

The Python client simplifies local setup with Solvio. For example, you can instantiate an in-memory version for testing or CI/CD workflows:

```python
from solvio_client import SolvioClient
solvio = SolvioClient(":memory:")
```

Or, opt for a disk-persisted version to support quick prototyping:

```python
client = SolvioClient(path="path/to/db")
```

### Client-Server

To unlock Solvio’s full potential on your local machine, start the container with this command:

```bash
docker run -p 6333:6333 solvio/solvio
```

Then, link up with any client, including the Python one, like so:

```python
solvio = SolvioClient("http://localhost:6333")
```

Before moving Solvio into a production environment, make sure to consult our [installation](https://trysolvio.com/documentation/guides/installation/index.html) and [security](https://trysolvio.com/documentation/guides/security/) guides.

### Clients

Solvio provides a range of client libraries to seamlessly weave it into your tech ecosystem:

### Where do I go from here?

- [Quick Start Guide](docs/QUICK_START.md)
- Comprehensive [Documentation](https://trysolvio.com/documentation/) serve as excellent launchpads

### Tap into Semantic Text Exploration 🔍

With Solvio, harness the strength of semantic embeddings to move past simple keyword lookups, revealing richer links within brief texts. Set up a neural-powered search in mere minutes using an existing neural model, and step into the next generation of text discovery.

### Dive into Visual Similarity Searches - Culinary Adventures 🍕

Exploration isn’t limited to words, especially in realms like food where visuals often outweigh descriptions. With Solvio, empower users to uncover their next tasty dish via image-based searches, no dish name required.

### Tackle Advanced Classification - E-commerce Product Sorting 📺

Step into the innovative world of extreme classification, a growing machine learning discipline dealing with multi-class and multi-label tasks across millions of categories. Leverage similarity learning techniques and see how a pre-trained transformer paired with Solvio can redefine product organization in online retail.

## Features

### Dynamic Filtering and Payload Handling

Solvio enables attaching any JSON payloads to vectors, facilitating both data retention and advanced filtering driven by payload contents. This system supports an array of data types and query options, from keyword searches and full-text analysis to numerical spans, geographic points, and beyond. Combine these filters flexibly with `should`, `must`, and `must_not` conditions to craft intricate business rules over similarity operations.

### Boosted Search with Sparse Vector Support

To tackle the shortcomings of dense vector embeddings for pinpoint keyword queries, Solvio integrates sparse vector capabilities alongside traditional dense ones. Think of sparse vectors as an evolved take on BM25 or TF-IDF scoring, enabling transformer-driven neural networks to prioritize tokens efficiently.

### Cost-Effective Vector Compression and Disk Storage

Solvio introduces multiple ways to streamline vector searches for better affordability and resource use. Its built-in quantization slashes RAM needs by up to 97%, while offering adjustable control over speed versus precision trade-offs.

### Robust Distributed Setup

Solvio supports extensive horizontal expansion through two core strategies:

1. Capacity growth through sharding and performance boosts via replication
2. Smooth, zero-interruption rolling updates and dynamic collection scaling

### Standout Capabilities

- **Smart Query Optimization and Payload Indexing** - Uses stored payload data to refine query performance.
- **SIMD-Enhanced Hardware Speed** - Taps into modern CPU designs (x86-x64 and Neon) for superior efficiency.
- **Asynchronous I/O Efficiency** - Leverages `io_uring` for peak disk throughput, even on networked storage.
- **Reliable Write-Ahead Logging** - Secures data consistency and update validation, even amidst power disruptions.

## License

Solvio operates under the Apache License, Version 2.0. Check out the [License file](https://github.com/solvio/solvio/blob/master/LICENSE) for details.
