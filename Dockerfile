FROM --platform=${BUILDPLATFORM:-linux/amd64} tonistiigi/xx AS xx
FROM --platform=${BUILDPLATFORM:-linux/amd64} lukemathwalker/cargo-chef:latest-rust-1.68.1 AS chef


FROM chef AS planner
WORKDIR /solvio
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder
WORKDIR /solvio

COPY --from=xx / /

RUN apt-get update \
    && apt-get install -y clang lld cmake protobuf-compiler \
    && rustup component add rustfmt

ARG TARGETPLATFORM
ENV TARGETPLATFORM=${TARGETPLATFORM:-linux/amd64}

RUN xx-apt-get install -y gcc g++ libc6-dev

ARG PROFILE=release
ARG FEATURES
ARG RUSTFLAGS
ARG LINKER=lld

COPY --from=planner /solvio/recipe.json recipe.json
RUN RUSTFLAGS="${LINKER:+-C link-arg=-fuse-ld=}$LINKER $RUSTFLAGS" \
    xx-cargo chef cook --profile $PROFILE ${FEATURES:+--features} $FEATURES --recipe-path recipe.json

COPY . .
RUN RUSTFLAGS="${LINKER:+-C link-arg=-fuse-ld=}$LINKER $RUSTFLAGS" \
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

WORKDIR ${APP}

ENV TZ=Etc/UTC \
    RUN_MODE=production

EXPOSE 6333
EXPOSE 6334

CMD ["./solvio"]
