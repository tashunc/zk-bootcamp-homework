FROM rust:slim-bullseye

RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

RUN noirup

WORKDIR /app

COPY . /app

