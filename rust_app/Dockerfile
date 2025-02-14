FROM rust:1.84-slim-bookworm

WORKDIR /rust_app

COPY . /rust_app

RUN rustc sc.rs -o sc

CMD ["./sc"]