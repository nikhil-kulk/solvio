# Cross-compiling using Docker multi-platform builds/images and `xx`.
#
# https://docs.docker.com/build/building/multi-platform/
# https://github.com/tonistiigi/xx
FROM --platform=${BUILDPLATFORM:-linux/amd64} tonistiigi/xx AS xx

# Utilizing Docker layer caching with `cargo-chef`.
#
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM --platform=${BUILDPLATFORM:-linux/amd64} lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef


FROM chef AS planner
WORKDIR /solvio
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder
WORKDIR /solvio

COPY --from=xx / /

# Relative order of `ARG` and `RUN` commands in the Dockerfile matters.
#
# If you pass a different `ARG` to `docker build`, it would invalidate Docker layer cache
# for the next steps. (E.g., the following steps may depend on a new `ARG` value, so Docker would
# have to re-execute them instead of using a cached layer from a previous run.)
#
# Steps in this stage are ordered in a way that should maximize Docker layer cache utilization,
# so, please, don't reorder them without prior consideration. 🥲

RUN apt-get update \
    && apt-get install -y clang lld cmake protobuf-compiler \
    && rustup component add rustfmt

# `ARG`/`ENV` pair is a workaround for `docker build` backward-compatibility.
#
# https://github.com/docker/buildx/issues/510
ARG BUILDPLATFORM
ENV BUILDPLATFORM=${BUILDPLATFORM:-linux/amd64}

ARG MOLD_VERSION=1.11.0

RUN case "$BUILDPLATFORM" in \
        */amd64 ) PLATFORM=x86_64 ;; \
        */arm64 | */arm64/* ) PLATFORM=aarch64 ;; \
        * ) echo "Unexpected BUILDPLATFORM '$BUILDPLATFORM'" >&2; exit 1 ;; \
    esac; \
    \
    mkdir -p /opt/mold; \
    cd /opt/mold; \
    \
    TARBALL="mold-$MOLD_VERSION-$PLATFORM-linux.tar.gz"; \
    curl -sSLO "https://github.com/rui314/mold/releases/download/v$MOLD_VERSION/$TARBALL"; \
    tar -xf "$TARBALL" --strip-components 1; \
    rm "$TARBALL"

# `ARG`/`ENV` pair is a workaround for `docker build` backward-compatibility.
#
# https://github.com/docker/buildx/issues/510
ARG TARGETPLATFORM
ENV TARGETPLATFORM=${TARGETPLATFORM:-linux/amd64}

RUN xx-apt-get install -y gcc g++ libc6-dev

# Select Cargo profile (e.g., `release`, `dev` or `ci`)
ARG PROFILE=release

# Enable crate features
ARG FEATURES

# Pass custom `RUSTFLAGS` (e.g., `--cfg tokio_unstable` to enable Tokio tracing/`tokio-console`)
ARG RUSTFLAGS

# Select linker (e.g., `mold`, `lld` or an empty string for the default linker)
ARG LINKER=mold

COPY --from=planner /solvio/recipe.json recipe.json
RUN PATH="$PATH:/opt/mold/bin" \
    RUSTFLAGS="${LINKER:+-C link-arg=-fuse-ld=}$LINKER $RUSTFLAGS" \
    xx-cargo chef cook --profile $PROFILE ${FEATURES:+--features} $FEATURES --recipe-path recipe.json

COPY . .
RUN PATH="$PATH:/opt/mold/bin" \
    RUSTFLAGS="${LINKER:+-C link-arg=-fuse-ld=}$LINKER $RUSTFLAGS" \
    xx-cargo build --profile $PROFILE ${FEATURES:+--features} $FEATURES --bin solvio \
    && PROFILE_DIR=$(if [ "$PROFILE" = dev ]; then echo debug; else echo $PROFILE; fi) \
    && mv target/$(xx-cargo --print-target-triple)/$PROFILE_DIR/solvio /solvio/solvio


FROM debian:11-slim AS solvio

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ARG APP=/solvio

RUN mkdir -p ${APP}

COPY --from=builder /solvio/solvio ${APP}/solvio
COPY --from=builder /solvio/config ${APP}/config
COPY --from=builder /solvio/tools/entrypoint.sh ${APP}/entrypoint.sh

WORKDIR ${APP}

ENV TZ=Etc/UTC \
    RUN_MODE=production

EXPOSE 6333
EXPOSE 6334

CMD ["./entrypoint.sh"]
