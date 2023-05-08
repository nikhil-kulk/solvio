# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM --platform=${BUILDPLATFORM:-linux/amd64} lukemathwalker/cargo-chef:latest-rust-1.69.0 AS chef
WORKDIR /solvio

ARG RUST_BUILD_PROFILE=ci

ARG MOLD_VERSION=1.11.0

# # Choose MOLD arch based on TARGETARCH: amd64 -> x86_64, arm64 -> aarch64
 COPY ./tools/mold_arch.sh ./mold_arch.sh
 
 RUN if [ "${RUST_BUILD_PROFILE}" ] = "ci"; then \
     curl -L https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}/mold-${MOLD_VERSION}-$(bash mold_arch.sh)-linux.tar.gz | tar zxf \
     && mv mold-${MOLD_VERSION}-$(bash mold_arch.sh)-linux /solvio/mold \
     && chmod +x /solvio/mold/bin/mold ; fi

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

# based on https://github.com/docker/buildx/issues/510
ARG TARGETARCH
ENV TARGETARCH=${TARGETARCH:-amd64}

WORKDIR /solvio

COPY ./tools/target_arch.sh ./target_arch.sh
RUN echo "Building for $TARGETARCH, arch: $(bash target_arch.sh)"

COPY --from=planner /solvio/recipe.json recipe.json

RUN apt-get update \
    && ( apt-get install -y gcc-multilib || echo "Warning: not installing gcc-multilib" ) \
    && apt-get install -y clang cmake gcc-aarch64-linux-gnu g++-aarch64-linux-gnu protobuf-compiler \
    && rustup component add rustfmt


RUN rustup target add $(bash target_arch.sh)

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --profile=${RUST_BUILD_PROFILE} --target $(bash target_arch.sh) --recipe-path recipe.json

COPY . .


# Build actual target here
RUN cargo build --profile=${RUST_BUILD_PROFILE} --target $(bash target_arch.sh) --bin solvio

RUN mv target/$(bash target_arch.sh)/${RUST_BUILD_PROFILE}/solvio /solvio/solvio

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
COPY --from=builder /solvio/tools/entrypoint.sh ${APP}/entrypoint.sh

WORKDIR ${APP}

CMD ["./entrypoint.sh"]
