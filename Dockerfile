FROM rust:latest

WORKDIR /usr/src

COPY ./src/main.rs ./src/main.rs
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build -r

EXPOSE 7878

CMD ["./target/release/load-balancer"]
