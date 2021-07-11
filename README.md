<p align="center">
  <img height="100" src="https://github.com/solvio/solvio/blob/master/docs/logo.svg?raw=true" alt="Solvio">
</p>

<p align="center">
    <b>Vector Similarity Search Engine with extended filtering support</b>
</p>


<p align=center>
    <a href="https://github.com/solvio/solvio/actions/workflows/rust.yml"><img src="https://github.com/solvio/solvio/workflows/Tests/badge.svg"></a>
    <a href="https://solvio.github.io/solvio/redoc/index.html"><img src="https://img.shields.io/badge/Docs-OpenAPI%203.0-success"></a>
    <a href="https://github.com/solvio/solvio/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-success"></a>
    <a href="https://t.me/joinchat/sIuUArGQRp9kMTUy"><img src="https://img.shields.io/badge/Telegram-Solvio-blue.svg?logo=telegram" alt="Telegram"></a>
</p>

Solvio (read: _quadrant_ ) is a vector similarity search engine.
It provides a production-ready service with a convenient API to store, search, and manage points - vectors with an additional payload.
Solvio is tailored to extended filtering support.  It makes it useful for all sorts of neural-network or semantic-based matching, faceted search, and other applications. 

Solvio is written in Rust :crab:, which makes it reliable even under high load.

With Solvio, embeddings or neural network encoders can be turned into full-fledged applications for matching, searching, recommending, and much more!

## Demo Projects

### Semantic Text Search :mag:

The neural search uses semantic embeddings instead of keywords and works best with short texts.
With Solvio and a pre-trained neural network, you can build and deploy semantic neural search on your data in minutes.
[Try it online!](https://demo.solvio.tech/)

### Similar Image Search - Food Discovery :pizza:

There are multiple ways to discover things, text search is not the only one.
In the case of food, people rely more on appearance than description and ingredients.
So why not let people choose their next lunch by its appearance, even if they don’t know the name of the dish?
[Check it out!](https://food-discovery.solvio.tech/)

### Extreme classification - E-commerce product categorisation :tv:

Extreme classification is a rapidly growing research area within machine learning focusing on multi-class and multi-label problems involving an extremely large number of labels.
Sometimes it is millions and tens of millions classes.
The most promising way to solve this problem is to use similarity learning models.
We put together a demo example of how you could approach the problem with a pre-trained transformer model and Solvio.
So you can [play with it online!](https://categories.solvio.tech/)


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

<table align="center">
    <tr>
        <td>
            <img width="300px" src="https://solvio.tech/content/images/chat_bots.png">
        </td>
        <td>
            <img width="300px" src="https://solvio.tech/content/images/matching_engines.png">
        </td>
    </tr>
    <tr>
        <td>
            Chat Bots
        </td>
        <td>
            Matching Engines
        </td>
    </tr>
</table>

</details>

## API

Online OpenAPI 3.0 documentation is available [here](https://solvio.github.io/solvio/redoc/index.html).
OpenAPI makes it easy to generate a client for virtually any framework or programing language.

You can also download raw OpenAPI [definitions](openapi/openapi.yaml).

## Features

### Filtering

Solvio supports key-value payload associated with vectors. It does not only store payload but also allows filter results based on payload values.
It allows any combinations of `should`, `must`, and `must_not` conditions, but unlike ElasticSearch post-filtering, Solvio guarantees all relevant vectors are retrieved.

### Rich data types

Vector payload supports a large variety of data types and query conditions, including string matching, numerical ranges, geo-locations, and more.
Payload filtering conditions allow you to build almost any custom business logic that should work on top of similarity matching.

### Query planning and payload indexes

Using the information about the stored key-value data, the `query planner` decides on the best way to execute the query.
For example, if the search space limited by filters is small, it is more efficient to use a full brute force than an index.

### SIMD Hardware Acceleration

With the `BLAS` library, Solvio can take advantage of modern CPU architectures. 
It allows you to search even faster on modern hardware.

### Write-ahead logging

Once the service confirmed an update - it won't lose data even in case of power shut down. 
All operations are stored in the update journal and the latest database state could be easily reconstructed at any moment.

### Stand-alone

Solvio does not rely on any external database or orchestration controller, which makes it very easy to configure.

## Usage

### Docker :whale:

Build your own from source

```bash
docker build . --tag=solvio
```

Or use latest pre-built image from [DockerHub](https://hub.docker.com/r/generall/solvio)

```bash
docker pull generall/solvio
```

To run container use command:

```bash
docker run -p 6333:6333 \
    -v $(pwd)/path/to/data:/solvio/storage \
    -v $(pwd)/path/to/custom_config.yaml:/solvio/config/production.yaml \
    solvio
```

* `/solvio/storage` - is a place where Solvio persists all your data. 
Make sure to mount it as a volume, otherwise docker will drop it with the container. 
* `/solvio/config/production.yaml` - is the file with engine configuration. You can override any value from the [reference config](config/config.yaml) 

Now Solvio should be accessible at [localhost:6333](http://localhost:6333/)

## Docs :notebook:

* The best place to start is [Quick Start Guide](QUICK_START.md)
* The [Documentation](https://solvio.tech/documentation/)
* Use the [OpenAPI specification](https://solvio.github.io/solvio/redoc/index.html) as a reference
* Follow our [Step-by-Step Tutorial](https://blog.solvio.tech/neural-search-tutorial-3f034ab13adc) to create your first neural network project with Solvio

## Contacts

* Join our [Telegram group](https://t.me/joinchat/sIuUArGQRp9kMTUy)
* Follow us on [Twitter](https://twitter.com/solvio_engine)
* Subscribe to our [Newsletters](https://tech.us1.list-manage.com/subscribe/post?u=69617d79374ac6280dd2230b2&amp;id=acb2b876fc)
* Write us an email [info@solvio.tech](mailto:info@solvio.tech)


## Contributors ✨

Thanks to the people who contributed to Solvio:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://t.me/neural_network_engineering"><img src="https://avatars.githubusercontent.com/u/1935623?v=4?s=50" width="50px;" alt=""/><br /><sub><b>Andrey Vasnetsov</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=generall" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/azayarni"><img src="https://avatars.githubusercontent.com/u/926368?v=4?s=50" width="50px;" alt=""/><br /><sub><b>Andre Zayarni</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=azayarni" title="Documentation">📖</a></td>
    <td align="center"><a href="http://www.linkedin.com/in/joanfontanalsmartinez/"><img src="https://avatars.githubusercontent.com/u/19825685?v=4?s=50" width="50px;" alt=""/><br /><sub><b>Joan Fontanals</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=JoanFM" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/trean"><img src="https://avatars.githubusercontent.com/u/7085263?v=4?s=50" width="50px;" alt=""/><br /><sub><b>trean</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=trean" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/kgrech"><img src="https://avatars.githubusercontent.com/u/9020133?v=4?s=50" width="50px;" alt=""/><br /><sub><b>Konstantin</b></sub></a><br /><a href="https://github.com/solvio/solvio/commits?author=kgrech" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Solvio is licensed under the Apache License, Version 2.0. View a copy of the [License file](LICENSE).
