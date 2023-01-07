FROM rust:latest

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . /app

RUN cargo build

EXPOSE 8013

CMD [ "trunk", "serve", "--address", "0.0.0.0", "--port", "8013"]