FROM rust:1.84-slim-bookworm

WORKDIR /rs_app

COPY . /rs_app

RUN rustc sc.rs -o sc

CMD ["./sc"]