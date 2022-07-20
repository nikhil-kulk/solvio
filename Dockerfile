# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 AS chef
WORKDIR /solvio

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

WORKDIR /solvio

COPY --from=planner /solvio/recipe.json recipe.json

RUN apt-get update ; apt-get install -y clang cmake ; rustup component add rustfmt

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build actual target here
RUN cargo build --release --bin solvio

FROM debian:11-slim
ARG APP=/solvio

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 6333
EXPOSE 6334

ENV TZ=Etc/UTC \
    RUN_MODE=production

RUN mkdir -p ${APP}

COPY --from=builder /solvio/target/release/solvio ${APP}/solvio
COPY --from=builder /solvio/config ${APP}/config

WORKDIR ${APP}

ENTRYPOINT ["./solvio"]
