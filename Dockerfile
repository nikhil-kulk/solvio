# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
ARG BUILDPLATFORM=linux/amd64
FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-1.63.0 AS chef
WORKDIR /solvio

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

ARG TARGETARCH=amd64

WORKDIR /solvio

COPY --from=planner /solvio/recipe.json recipe.json

RUN apt-get update && apt-get install -y gcc-multilib && apt-get install -y clang cmake gcc-aarch64-linux-gnu g++-aarch64-linux-gnu && rustup component add rustfmt

COPY ./tools/target_arch.sh ./target_arch.sh
RUN rustup target add $(bash target_arch.sh)

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target $(bash target_arch.sh) --recipe-path recipe.json

COPY . .

# Build actual target here
RUN cargo build --release --target $(bash target_arch.sh) --bin solvio

RUN mv target/$(bash target_arch.sh)/release/solvio /solvio/solvio

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

COPY --from=builder /solvio/solvio ${APP}/solvio
COPY --from=builder /solvio/config ${APP}/config

WORKDIR ${APP}

CMD ["./solvio"]
