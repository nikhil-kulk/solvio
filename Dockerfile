FROM rust:1.51 as builder

COPY . ./solvio
WORKDIR ./solvio

ENV OPENBLAS_DYNAMIC_ARCH="1"
RUN apt-get update ; apt-get install -y clang libopenblas-dev libgfortran-8-dev gfortran

# Build actual target here
RUN cargo build --release --bin solvio

FROM debian:buster-slim
ARG APP=/solvio

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 6333
EXPOSE 6334

ENV TZ=Etc/UTC \
    RUN_MODE=production \
    OPENBLAS_NUM_THREADS=1

RUN mkdir -p ${APP}

COPY --from=builder /solvio/target/release/solvio ${APP}/solvio
COPY --from=builder /solvio/config ${APP}/config

WORKDIR ${APP}

CMD ["./solvio"]
