FROM rust:1.93.1-alpine

WORKDIR /app

COPY . .

RUN cargo build

CMD ["cargo", "run", "-p", "bid-api"]