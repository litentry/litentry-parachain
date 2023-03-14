# global args that are used across multiple stages
ARG PROFILE

# ==========================
# stage 1: build
# https://hub.docker.com/_/rust
# our host is based on bullseye/sid => rust:latest
# ==========================
FROM rust:latest as builder

WORKDIR /litentry
COPY . /litentry

RUN apt-get update && apt-get install -yq clang libclang-dev cmake protobuf-compiler

ARG BUILD_ARGS
ARG PROFILE

RUN cargo build --locked --profile $PROFILE $BUILD_ARGS

# ==========================
# stage 2: packaging
# ==========================
FROM ubuntu:20.04
LABEL maintainer="litentry-dev"

ARG PROFILE

COPY --from=builder /litentry/target/$PROFILE/litentry-collator /usr/local/bin

# install netcat for healthcheck
RUN apt-get update && apt-get install -yq netcat && cp /usr/bin/nc /usr/local/bin/

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
