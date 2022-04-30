FROM rust:latest

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

RUN cargo install  wasm-bindgen-cli

EXPOSE 3000

WORKDIR /app

CMD [ "trunk", "serve" ]


