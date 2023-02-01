# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY src /code

FROM base AS development
EXPOSE 8000
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder
RUN cargo build --release --offline

FROM debian:buster-slim as production
ENV ROCKET_ENV=production
EXPOSE 8000
COPY --from=builder /code/target/release/rust-restapi /rust-restapi
CMD [ "/rust-restapi" ]