FROM rust:1.59 as builder

COPY . ./solvio
WORKDIR ./solvio

RUN apt-get update ; apt-get install -y clang

# Build actual target here
RUN cargo build --release --bin solvio

FROM debian:bullseye-slim
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

CMD ["./solvio"]
