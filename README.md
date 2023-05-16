<p align="center">
  <img height="100" src="https://github.com/solvio/solvio/raw/master/docs/logo.svg" alt="Solvio">
</p>

<p align="center">
    <b>Vector Search Engine for the next generation of AI applications</b>
</p>

<p align=center>
    <a href="https://github.com/solvio/solvio/actions/workflows/rust.yml"><img src="https://github.com/solvio/solvio/workflows/Tests/badge.svg" alt="Tests status"></a>
    <a href="https://solvio.github.io/solvio/redoc/index.html"><img src="https://img.shields.io/badge/Docs-OpenAPI%203.0-success" alt="OpenAPI Docs"></a>
    <a href="https://github.com/solvio/solvio/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-success" alt="Apache 2.0 License"></a>
    <a href="https://solvio.to/discord"><img src="https://img.shields.io/badge/Discord-Solvio-5865F2.svg?logo=discord" alt="Discord"></a>
    <a href="https://solvio.to/roadmap"><img src="https://img.shields.io/badge/Roadmap-2023-bc1439.svg" alt="Roadmap 2023"></a>
</p>

**Solvio** (read: _quadrant_) is a vector similarity search engine and vector database.
It provides a production-ready service with a convenient API to store, search, and manage points—vectors with an additional payload
Solvio is tailored to extended filtering support. It makes it useful for all sorts of neural-network or semantic-based matching, faceted search, and other applications.

Solvio is written in Rust 🦀, which makes it fast and reliable even under high load. See [benchmarks](https://solvio.tech/benchmarks/).

With Solvio, embeddings or neural network encoders can be turned into full-fledged applications for matching, searching, recommending, and much more!

<p align="center">
<strong><a href="./QUICK_START.md">Quick Start</a> • <a href="#clients">Client Libraries</a> • <a href="#demo-projects">Demo Projects</a> • <a href="#integrations">Integrations</a> • <a href="#contacts">Contact</a>

</strong>
</p>

## Getting Started

### Python

```
pip install solvio-client
```

The python client offers a convenient way to start with Solvio locally: 

```python
from solvio_client import SolvioClient
solvio = SolvioClient(":memory:") # Create in-memory Solvio instance, for testing, CI/CD
# OR
client = SolvioClient(path="path/to/db")  # Persists changes to disk, fast prototyping
```

### Client-Server

This is the recommended method for production usage. To run the container, use the command:

```bash
docker run -p 6333:6333 solvio/solvio
```

Now you can connect to this with any client, including Python:

```python
solvio = SolvioClient("http://localhost:6333") # Connect to existing Solvio instance, for production
```

### Clients

Solvio offers the following client libraries to help you integrate it into your application stack with ease:

- Official: [Go client](https://github.com/solvio/go-client)
- Official: [Rust client](https://github.com/solvio/rust-client)
- Official: [JavaScript/TypeScript client](https://github.com/solvio/solvio-js)
- Official: [Python client](https://github.com/solvio/solvio-client)
- Community: [Elixir](https://hexdocs.pm/solvio/readme.html)
- Community: [PHP](https://github.com/hkulekci/solvio-php)
- Community: [Ruby](https://github.com/andreibondarev/solvio-ruby)
- Community: [Java](https://github.com/metaloom/solvio-java-client)

### Where do I go from here?

- [Quick Start Guide](https://github.com/solvio/solvio/blob/master/QUICK_START.md)
- End to End [Colab Notebook](https://colab.research.google.com/drive/1Bz8RSVHwnNDaNtDwotfPj0w7AYzsdXZ-?usp=sharing) demo with SentenceBERT and Solvio
- Detailed [Documentation](https://solvio.tech/documentation/) are great starting points
- [Step-by-Step Tutorial](https://solvio.to/solvio-tutorial) to create your first neural network project with Solvio

## Demo Projects

### Discover Semantic Text Search 🔍

Unlock the power of semantic embeddings with Solvio, transcending keyword-based search to find meaningful connections in short texts. Deploy a neural search in minutes using a pre-trained neural network, and experience the future of text search. [Try it online!](https://solvio.to/semantic-search-demo)

### Explore Similar Image Search - Food Discovery 🍕

There's more to discovery than text search, especially when it comes to food. People often choose meals based on appearance rather than descriptions and ingredients. Let Solvio help your users find their next delicious meal using visual search, even if they don't know the dish's name. [Check it out!](https://solvio.to/food-discovery)

### Master Extreme Classification - E-commerce Product Categorization 📺

Enter the cutting-edge realm of extreme classification, an emerging machine learning field tackling multi-class and multi-label problems with millions of labels. Harness the potential of similarity learning models, and see how a pre-trained transformer model and Solvio can revolutionize e-commerce product categorization. [Play with it online!](https://solvio.to/extreme-classification-demo)

<details>
<summary> More solutions </summary>

<table>
    <tr>
        <td width="30%">
            <img src="https://solvio.tech/content/images/text_search.png">
        </td>
        <td width="30%">
            <img src="https://solvio.tech/content/images/image_search.png">
        </td>
        <td width="30%">
            <img src="https://solvio.tech/content/images/recommendations.png">
        </td>
    </tr>
    <tr>
        <td>
            Semantic Text Search
        </td>
        <td>
            Similar Image Search
        </td>
        <td>
            Recommendations
        </td>
    </tr>
</table>

<table>
    <tr>
        <td>
            <img width="300px" src="https://solvio.tech/content/images/chat_bots.png">
        </td>
        <td>
            <img width="300px" src="https://solvio.tech/content/images/matching_engines.png">
        </td>
        <td>
            <img width="300px" src="https://solvio.tech/content/images/anomalies_detection.png">
        </td>
    </tr>
    <tr>
        <td>
            Chat Bots
        </td>
        <td>
            Matching Engines
        </td>
        <td>
            Anomaly Detection
        </td>
    </tr>
</table>

</details>

## API

### REST

Online OpenAPI 3.0 documentation is available [here](https://solvio.github.io/solvio/redoc/index.html).
OpenAPI makes it easy to generate a client for virtually any framework or programming language.

You can also download raw OpenAPI [definitions](https://github.com/solvio/solvio/blob/master/docs/redoc/master/openapi.json).

### gRPC

For faster production-tier searches, Solvio also provides a gRPC interface. You can find gRPC documentation [here](https://solvio.tech/documentation/quick_start/#grpc).

## Features

### Filtering and Payload

Solvio enables JSON payloads to be associated with vectors, providing both storage and filtering based on payload values. It supports various combinations of `should`, `must`, and `must_not` conditions, ensuring retrieval of all relevant vectors unlike `ElasticSearch` post-filtering.

### Rich Data Types

The vector payload accommodates diverse data types and query conditions, including string matching, numerical ranges, geo-locations, and more. These filtering conditions empower you to create custom business logic on top of similarity matching.

### Query Planning and Payload Indexes

The _query planner_ leverages stored payload information to optimize query execution. For instance, smaller search spaces limited by filters might benefit from full brute force over an index.

### SIMD Hardware Acceleration

Utilizing modern CPU x86-x64 architectures, Solvio delivers faster search performance on modern hardware.

### Write-Ahead Logging

Solvio ensures data persistence with update confirmation, even during power outages. The update journal stores all operations, enabling effortless reconstruction of the latest database state.

### Distributed Deployment

As of [v0.8.0](https://github.com/solvio/solvio/releases/tag/v0.8.0), Solvio supports distributed deployment. Multiple Solvio machines form a cluster for horizontal scaling, coordinated through the [Raft](https://raft.github.io/) protocol.

### Stand-alone

Solvio operates independently, without reliance on external databases or orchestration controllers, simplifying configuration.

# Integrations

Examples and/or documentation of Solvio integrations:

- [Cohere](https://docs.cohere.com/docs/integrations#solvio) ([blogpost on building a QA app with Cohere and Solvio](https://solvio.tech/articles/qa-with-cohere-and-solvio/)) - Use Cohere embeddings with Solvio
- [DocArray](https://docarray.jina.ai/advanced/document-store/solvio/) - Use Solvio as a document store in DocArray
- [LangChain](https://python.langchain.com/en/latest/ecosystem/solvio.html) ([blogpost](https://solvio.tech/articles/langchain-integration/)) - Use Solvio as a memory backend for LangChain.
- [LlamaIndex](https://gpt-index.readthedocs.io/en/latest/reference/indices/composability_query.html#gpt_index.data_structs.struct_type.IndexStructType.solvio) - Use Solvio as a memory backend for LlamaIndex.
- [OpenAI - ChatGPT retrieval plugin](https://github.com/openai/chatgpt-retrieval-plugin/blob/main/docs/providers/solvio/setup.md) - Use Solvio as a memory backend for ChatGPT
- [Microsoft Semantic Kernel](https://devblogs.microsoft.com/semantic-kernel/the-power-of-persistent-memory-with-semantic-kernel-and-solvio-vector-database/) - Use Solvio as persistent memory with Semantic Kernel 

## Contacts

- Have questions? Join our [Discord channel](https://solvio.to/discord) or mention [@solvio_engine on Twitter](https://solvio.to/twitter)
- Want to stay in touch with latest releases? Subscribe to our [Newsletters](https://solvio.to/newsletter)
- Looking for a managed cloud? Check [pricing](https://solvio.tech/pricing/), need something personalised? We're at [info@solvio.tech](mailto:info@solvio.tech)

## Contributors ✨

Thanks to the people who contributed to Solvio:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center"><a href="https://t.me/neural_network_engineering"><img src="https://avatars.githubusercontent.com/u/1935623?v=4?s=50" width="50px;" alt="Andrey Vasnetsov"/><br /><sub><b>Andrey Vasnetsov</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=generall" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/azayarni"><img src="https://avatars.githubusercontent.com/u/926368?v=4?s=50" width="50px;" alt="Andre Zayarni"/><br /><sub><b>Andre Zayarni</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=azayarni" title="Documentation">📖</a></td>
      <td align="center"><a href="http://www.linkedin.com/in/joanfontanalsmartinez/"><img src="https://avatars.githubusercontent.com/u/19825685?v=4?s=50" width="50px;" alt="Joan Fontanals"/><br /><sub><b>Joan Fontanals</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=JoanFM" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/trean"><img src="https://avatars.githubusercontent.com/u/7085263?v=4?s=50" width="50px;" alt="trean"/><br /><sub><b>trean</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=trean" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/kgrech"><img src="https://avatars.githubusercontent.com/u/9020133?v=4?s=50" width="50px;" alt="Konstantin"/><br /><sub><b>Konstantin</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=kgrech" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/kekonen"><img src="https://avatars.githubusercontent.com/u/11177808?v=4?s=50" width="50px;" alt="Daniil Naumetc"/><br /><sub><b>Daniil Naumetc</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=kekonen" title="Code">💻</a></td>
      <td align="center"><a href="https://dev.to/vearutop"><img src="https://avatars.githubusercontent.com/u/1381436?v=4?s=50" width="50px;" alt="Viacheslav Poturaev"/><br /><sub><b>Viacheslav Poturaev</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=vearutop" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center"><a href="https://github.com/galibey"><img src="https://avatars.githubusercontent.com/u/48586936?v=4?s=50" width="50px;" alt="Alexander Galibey"/><br /><sub><b>Alexander Galibey</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=galibey" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/HaiCheViet"><img src="https://avatars.githubusercontent.com/u/37202591?v=4?s=50" width="50px;" alt="HaiCheViet"/><br /><sub><b>HaiCheViet</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=HaiCheViet" title="Code">💻</a></td>
      <td align="center"><a href="https://tranzystorek-io.github.io/"><img src="https://avatars.githubusercontent.com/u/5671049?v=4?s=50" width="50px;" alt="Marcin Puc"/><br /><sub><b>Marcin Puc</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=tranzystorek-io" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/anveq"><img src="https://avatars.githubusercontent.com/u/94402218?v=4?s=50" width="50px;" alt="Anton V."/><br /><sub><b>Anton V.</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=anveq" title="Code">💻</a></td>
      <td align="center"><a href="http://agourlay.github.io"><img src="https://avatars.githubusercontent.com/u/606963?v=4?s=50" width="50px;" alt="Arnaud Gourlay"/><br /><sub><b>Arnaud Gourlay</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=agourlay" title="Code">💻</a></td>
      <td align="center"><a href="https://t.me/type_driven_thoughts"><img src="https://avatars.githubusercontent.com/u/17401538?v=4?s=50" width="50px;" alt="Egor Ivkov"/><br /><sub><b>Egor Ivkov</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=eadventurous" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/IvanPleshkov"><img src="https://avatars.githubusercontent.com/u/20946825?v=4?s=50" width="50px;" alt="Ivan Pleshkov"/><br /><sub><b>Ivan Pleshkov</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=IvanPleshkov" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center"><a href="https://github.com/daniilsunyaev"><img src="https://avatars.githubusercontent.com/u/3955599?v=4?s=50" width="50px;" alt="Daniil"/><br /><sub><b>Daniil</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=daniilsunyaev" title="Code">💻</a></td>
      <td align="center"><a href="http://homeonrails.com"><img src="https://avatars.githubusercontent.com/u/1282182?v=4?s=50" width="50px;" alt="Anton Kaliaev"/><br /><sub><b>Anton Kaliaev</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=melekes" title="Code">💻</a></td>
      <td align="center"><a href="https://soundcloud.com/norom"><img src="https://avatars.githubusercontent.com/u/7762532?v=4?s=50" width="50px;" alt="Andre Julius"/><br /><sub><b>Andre Julius</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=NotNorom" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/prok20"><img src="https://avatars.githubusercontent.com/u/20628026?v=4?s=50" width="50px;" alt="Prokudin Alexander"/><br /><sub><b>Prokudin Alexander</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=prok20" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/elbart"><img src="https://avatars.githubusercontent.com/u/48974?v=4?s=50" width="50px;" alt="Tim Eggert"/><br /><sub><b>Tim Eggert</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=elbart" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/gvelo"><img src="https://avatars.githubusercontent.com/u/943360?v=4?s=50" width="50px;" alt="Gabriel Velo"/><br /><sub><b>Gabriel Velo</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=gvelo" title="Code">💻</a></td>
      <td align="center"><a href="http://burtonqin.github.io"><img src="https://avatars.githubusercontent.com/u/11943383?v=4?s=50" width="50px;" alt="Boqin Qin(秦 伯钦)"/><br /><sub><b>Boqin Qin(秦 伯钦)</b></sub></a><br /><a href="https://github.com/solvio/solvio/issues?q=author%3ABurtonQin" title="Bug reports">🐛</a></td>
    </tr>
    <tr>
      <td align="center"><a href="https://forloop.co.uk/blog"><img src="https://avatars.githubusercontent.com/u/208231?v=4?s=50" width="50px;" alt="Russ Cam"/><br /><sub><b>Russ Cam</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=russcam" title="Code">💻</a></td>
      <td align="center"><a href="https://github.com/erare-humanum"><img src="https://avatars.githubusercontent.com/u/116254494?v=4?s=50" width="50px;" alt="erare-humanum"/><br /><sub><b>erare-humanum</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=erare-humanum" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Solvio is licensed under the Apache License, Version 2.0. View a copy of the [License file](https://github.com/solvio/solvio/blob/master/LICENSE).
