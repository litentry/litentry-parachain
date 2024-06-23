# global args that are used across multiple stages
ARG PROFILE

# ==========================
# stage 1: build
# https://hub.docker.com/_/rust
# our host is based on bullseye/sid (ubuntu 20)
# ==========================
FROM rust:bullseye as builder

WORKDIR /litentry
COPY . /litentry

RUN apt-get update && apt-get install -yq clang libclang-dev cmake protobuf-compiler

ARG BUILD_ARGS
ARG PROFILE

RUN cargo build --locked --profile $PROFILE $BUILD_ARGS

# ==========================
# stage 2: packaging
# ==========================
FROM ubuntu:22.04 AS parachain
LABEL maintainer="Trust Computing GmbH <info@litentry.com>"

ARG PROFILE

COPY --from=builder /litentry/target/$PROFILE/litentry-collator /usr/local/bin

# install netcat for healthcheck
RUN apt-get update && \
    apt-get install -yq netcat ca-certificates && \
    update-ca-certificates && \
    apt-get clean && \
    rm -rf /var/cache/apt/lists && \
    cp /usr/bin/nc /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /litentry litentry && \
    mkdir -p /data /litentry/.local/share && \
    chown -R litentry:litentry /data && \
    ln -s /data /litentry/.local/share/litentry-collator && \
    # check if executable works in this container
    /usr/local/bin/litentry-collator --version

USER litentry
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/litentry-collator"]
CMD ["--help"]

FROM ubuntu:22.04 AS chain-aio
LABEL maintainer="Trust Computing GmbH <info@litentry.com>"

ARG PROFILE
ENV NVM_DIR /opt/nvm

# install netcat for healthcheck
RUN apt-get update && \
    apt install -yq build-essential ocaml ocamlbuild automake autoconf libtool wget \
    python-is-python3 libssl-dev git cmake perl python3 dkms pip python3-click clang-13 \
    lldb-13 lld-13 clangd net-tools libssl-dev libcurl4-openssl-dev protobuf-compiler \
    libprotobuf-dev debhelper cmake reprepro unzip pkgconf libboost-dev libboost-system-dev \
    libboost-thread-dev lsb-release libsystemd0 pkgconf jq python3-pip curl && \
    apt-get install -yq ca-certificates && \
    update-ca-certificates && \
    wget -O /usr/local/bin/polkadot -q https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot && \
    chmod +x /usr/local/bin/polkadot && \
    curl -fsSL https://deb.nodesource.com/setup_21.x | sudo -E bash - && \
    apt-get update && apt-get install -y nodejs && \
    npm install -g pnpm && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
    apt-key del 23E7166788B63E1E && \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    apt update && apt install -y yarn  && \
    mkdir -p $NVM_DIR && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash && \
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion" && \
    nvm install 18 && \
    nvm use 18 && \
    apt-get clean && \
    rm -rf /var/cache/apt/lists 

RUN useradd -m -u 1000 -U -s /bin/sh -d /litentry litentry && \
	mkdir -p /opt/litentry/parachain /code/litentry-parachain && \
	chown -R litentry:litentry /opt/litentry
	
COPY --from=builder /litentry/target/$PROFILE/litentry-collator /usr/local/bin
RUN chmod +x /usr/local/bin/litentry-collator && \
    # check if executable works in this container
    /usr/local/bin/litentry-collator --version

COPY ./node/src/chain_specs /code/litentry-parachain/node/src/chain_specs
COPY ./ts-tests /code/litentry-parachain/ts-tests 
RUN chown -R litentry:litentry /code

COPY ./docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

USER litentry

EXPOSE 9615 9933 9936 9937 9944 9946 9947 30332 30333 30336 30337
VOLUME ["/opt/litentry"]

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]