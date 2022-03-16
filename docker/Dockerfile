FROM rust:latest as chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly
RUN cargo chef prepare --recipe-path recipe.json

# ==========================
# stage 1: build
# https://hub.docker.com/_/rust
# our host is based on bullseye/sid => rust:latest
# ==========================
FROM chef as builder
WORKDIR /litentry
RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

COPY --from=planner /app/recipe.json /litentry/recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . /litentry

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

ARG BUILD_ARGS

RUN cargo build --release $BUILD_ARGS

# ==========================
# stage 2: packaging
# ==========================

FROM ubuntu:20.04
LABEL maintainer="litentry-dev"

COPY --from=builder /litentry/target/release/litentry-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /litentry litentry && \
	mkdir -p /data /litentry/.local/share && \
	chown -R litentry:litentry /data && \
	ln -s /data /litentry/.local/share/litentry-collator && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/litentry-collator --version

USER litentry
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/litentry-collator"]
CMD ["--help"]
