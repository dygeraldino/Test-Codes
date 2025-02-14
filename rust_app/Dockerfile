FROM rust:1.84.1-slim-bookworm

WORKDIR /rust_app

COPY . /rust_app

RUN rustc sc.rs -o sc

CMD ["./sc"]