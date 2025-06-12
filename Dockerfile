FROM rust:1.87-bookworm AS builder

WORKDIR /app

COPY ./crates/backend ./crates/backend
COPY ./crates/ai ./crates/ai
COPY ./crates/model ./crates/model
COPY Cargo.* ./

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

COPY --from=builder /app/target/release/backend /

CMD ["./backend"]
