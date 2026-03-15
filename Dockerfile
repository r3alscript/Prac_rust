FROM rust:1.93-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY shared/Cargo.toml ./shared/Cargo.toml
COPY bid-api/Cargo.toml ./bid-api/Cargo.toml
COPY bid-worker/Cargo.toml ./bid-worker/Cargo.toml
COPY benchmark-generator/Cargo.toml ./benchmark-generator/Cargo.toml
COPY benchmark-worker/Cargo.toml ./benchmark-worker/Cargo.toml
COPY benchmark-bashboard/Cargo.toml ./benchmark-bashboard/Cargo.toml
COPY benchmark-api/Cargo.toml ./benchmark-api/Cargo.toml
COPY shared/build.rs ./shared/build.rs
COPY shared/src/proto ./shared/src/proto

RUN mkdir -p \
    shared/src \
    bid-api/src \
    bid-worker/src \
    benchmark-generator/src \
    benchmark-worker/src \
    benchmark-bashboard/src \
    benchmark-api/src && \
    [ -f shared/src/lib.rs ] || printf '%s\n' '// dummy lib' > shared/src/lib.rs && \
    printf '%s\n' 'fn main() {}' > bid-api/src/main.rs && \
    printf '%s\n' 'fn main() {}' > bid-worker/src/main.rs && \
    printf '%s\n' 'fn main() {}' > benchmark-generator/src/main.rs && \
    printf '%s\n' 'fn main() {}' > benchmark-worker/src/main.rs && \
    printf '%s\n' 'fn main() {}' > benchmark-bashboard/src/main.rs && \
    printf '%s\n' 'fn main() {}' > benchmark-api/src/main.rs

RUN cargo build -p bid-api -p bid-worker

COPY . .

RUN cargo build -p bid-api -p bid-worker

FROM debian:bookworm-slim AS bid-api

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/debug/bid-api /usr/local/bin/bid-api

EXPOSE 8080

CMD ["bid-api"]

FROM debian:bookworm-slim AS bid-worker

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/debug/bid-worker /usr/local/bin/bid-worker

CMD ["bid-worker"]