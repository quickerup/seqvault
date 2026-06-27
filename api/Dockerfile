FROM rust:1.81 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin/api
CMD ["api"]
