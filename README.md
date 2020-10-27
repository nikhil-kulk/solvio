# :black_square_button: Solvio

Solvio (read: _quadrant_ ) is a vector similarity search engine.
It provides a production-ready service with a convenient API to store, search, and manage points (vectors with additional payload).

It is tailored on extended support of filtering, which makes it useful for all sorts of neural-network or semantic based matching, faceted search, and other applications. 

Solvio is written in Rust, which makes it reliable even under high load.

# API [![OpenAPI docs](https://img.shields.io/badge/docs-OpenAP3.0-success)](https://solvio.github.io/solvio/redoc/index.html)

Online OpenAPI 3.0 documentation is available [here](https://solvio.github.io/solvio/redoc/index.html).
OpenAPI makes it easy to generate client for virtually any framework or programing language.

You can also download raw OpenAPI [definitions](openapi/openapi.yaml).

# Features

## Filtering

Solvio supports any combinations of `should`, `must` and `must_not` conditions,
which makes it possible to use in applications when object could not be described solely by vector.
It could be location features, availability flags, and other custom properties businesses should take into account.

## Write-ahead logging

Once service confirmed an update - it won't lose data even in case of power shut down. 
All operations are stored in the update journal and the latest database state could be easily reconstructed at any moment.

## Stand-alone

Solvio does not rely on any external database or orchestration controller, which makes it very easy to configure.

# Usage

## Docker

