FROM --platform=${BUILDPLATFORM:-linux/amd64} lukemathwalker/cargo-chef:latest-rust-1.67.1 AS chef
WORKDIR /echo

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

# based on https://github.com/docker/buildx/issues/510
ARG TARGETARCH
ENV TARGETARCH=${TARGETARCH:-amd64}

WORKDIR /echo

COPY ../tools/target_arch.sh ./target_arch.sh
RUN echo "Building for $TARGETARCH, arch: $(bash target_arch.sh)"

COPY --from=planner /echo/recipe.json recipe.json

RUN apt-get update && apt-get install -y gcc-multilib && apt-get install -y clang cmake gcc-aarch64-linux-gnu g++-aarch64-linux-gnu protobuf-compiler && rustup component add rustfmt


RUN rustup target add $(bash target_arch.sh)

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target $(bash target_arch.sh) --recipe-path recipe.json

COPY . .

# Build actual target here
RUN cargo build --release --target $(bash target_arch.sh) --bin solvio

RUN mv target/$(bash target_arch.sh)/release/echo /echo

FROM debian:11-slim

EXPOSE 8080
EXPOSE 8081

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /echo /echo

WORKDIR /

CMD ["./echo"]
