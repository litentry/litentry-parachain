FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Polkadot. Here we create the binary."

ARG PROFILE=release
WORKDIR /litentry-parachain

COPY . /litentry-parachain

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the litentry-collator binary."
ARG PROFILE=release
COPY --from=builder /litentry-parachain/target/$PROFILE/litentry-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /litentry-parachain litentry && \
	mkdir -p /litentry-parachain/.local/share && \
	mkdir /data && \
	ln -s /data /litentry-parachain/.local/share/litentry-parachain && \
	rm -rf /usr/bin /usr/sbin

EXPOSE 30333 9933 9944 40333
VOLUME ["/data"]

CMD ["/usr/local/bin/litentry-collator"]
