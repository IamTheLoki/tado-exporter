FROM rust:slim-bookworm AS builder

RUN apt update && \
    apt install -y ca-certificates libssl-dev libfindbin-libs-perl make && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/tado-exporter

COPY Cargo.* .
COPY src/ ./src

RUN rustup toolchain install stable && \
    cargo build --release && \
    cp target/release/tado-exporter /tado-exporter

FROM debian:bookworm-slim
LABEL name="tado-exporter"

RUN apt update && \
    apt install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /tado-exporter /usr/bin/tado-exporter

CMD ["tado-exporter"]