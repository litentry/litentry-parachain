all:
	@echo "make all not implemented"

## variant declaration

NODE_BIN=litentry-collator
RUNTIME=litentry-parachain-runtime

## build release

.PHONY: build-all
build-all:
	cargo build --release

.PHONY: build-node
build-node:
	cargo build -p $(call pkgid, $(NODE_BIN)) --release

# TODO: use srtool to build wasm
.PHONY: build-runtime
build-runtime:
	cargo build -p $(call pkgid, $(RUNTIME)) --release

.PHONY: build-docker
build-docker:
	@./scripts/build-docker.sh

.PHONY: build-spec-dev
build-spec-dev:
	./target/release/$(NODE_BIN) build-spec --chain dev --disable-default-bootnode > ./source/local.json

.PHONY: build-benchmark
build-benchmark:
	cargo build --features runtime-benchmarks --release

## test

.PHONY: test-all
test-all:
	cargo test

.PHONY: test-node
test-node:
	cargo test --package $(call pkgid, $(NODE_BIN))

.PHONY: test-ci
test-ci: launch-local-docker
	@./scripts/run-ci-test.sh

## format

.PHONY: format
format:
	cargo fmt --all -- --check

# launch a local dev network using docker
.PHONY: launch-local-docker
launch-local-docker:
	@cd docker/generated-dev; docker-compose up -d --build

# stop the local dev containers and cleanup images
# for the most part used when done with launch-local-docker
.PHONY: clean-local-docker
clean-local-docker:
	@./scripts/clean-local-docker.sh
	
## generate docker-compose files

.PHONY: generate-docker-compose-dev
generate-docker-compose-dev:
	@./scripts/generate-docker-files.sh dev

.PHONY: generate-docker-compose-staging
generate-docker-compose-staging:
	@./scripts/generate-docker-files.sh staging

## benchmark

.PHONY: benchmark-frame-system
benchmark-frame-system:
	target/release/litentry-collator benchmark \
	--chain=./source/local.json \
	--execution=wasm  \
	--db-cache=20 \
	--wasm-execution=compiled \
	--pallet=frame_system \
	--extrinsic=* \
	--heap-pages=4096 \
	--steps=20 \
	--repeat=50 \
	--output=./source/weights.rs \
	--template=./.maintain/frame-weight-template.hbs

.PHONY: benchmark-account-linker
benchmark-account-linker:
	target/release/litentry-collator benchmark \
	--chain=./source/local.json \
	--execution=wasm  \
	--db-cache=20 \
	--wasm-execution=compiled \
	--pallet=pallet-account-linker \
	--extrinsic=* \
	--heap-pages=4096 \
	--steps=20 \
	--repeat=50 \
	--output=./source/weights.rs \
	--template=./.maintain/frame-weight-template.hbs

.PHONY: benchmark-offchain-worker
benchmark-offchain-worker:
	target/release/litentry-collator benchmark \
	--chain=./source/local.json \
	--execution=wasm  \
	--db-cache=20 \
	--wasm-execution=compiled \
	--pallet=pallet-offchain-worker \
	--extrinsic=* \
	--heap-pages=4096 \
	--steps=20 \
	--repeat=50 \
	--output=./source/weights.rs \
	--template=./.maintain/frame-weight-template.hbs

.PHONY: benchmark-nft
benchmark-nft:
	target/release/litentry-collator benchmark \
	--chain=./source/local.json \
	--execution=wasm  \
	--db-cache=20 \
	--wasm-execution=compiled \
	--pallet=pallet-nft \
	--extrinsic=* \
	--heap-pages=4096 \
	--steps=20 \
	--repeat=50 \
	--output=./source/weights.rs \
	--template=./.maintain/frame-weight-template.hbs

define pkgid
	$(shell cargo pkgid $1)
endef
