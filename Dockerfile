FROM rust:1.90

COPY ./ ./app

WORKDIR /app

RUN cargo build --release

EXPOSE 8404

CMD ["./target/release/waves"]
