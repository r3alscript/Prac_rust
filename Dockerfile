<<<<<<< HEAD
FROM rust:1.93.1-alpine

WORKDIR /app

COPY . .

RUN cargo build

=======
FROM rust:1.93.1-alpine

WORKDIR /app

COPY . .

RUN cargo build

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
CMD ["cargo", "run", "-p", "bid-api"]