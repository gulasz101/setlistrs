FROM rust:latest as build

ENV DATABASE_URL=sqlite:///database/setlistrs.database

WORKDIR /setlistrs
COPY . .
WORKDIR setlistrs-server

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo-binstall -y sqlx-cli

RUN mkdir /database

RUN sqlx database create
RUN sqlx migrate run

RUN cargo build --release

VOLUME /database

EXPOSE 8081

CMD ["./target/release/setlistrs-server"]
