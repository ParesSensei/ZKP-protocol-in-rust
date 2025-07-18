FROM rustlang/rust:nightly

WORKDIR /zkp-server

COPY . .

RUN apt update
RUN apt install -y protobuf-compiler

RUN cargo +nightly build --release --bin server --bin client
