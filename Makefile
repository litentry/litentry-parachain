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

.PHONY: build-runtime
build-runtime:
	cargo build -p $(call pkgid, $(RUNTIME)) --release

# use srtool to build wasm locally
# in github actions srtool-actions is used
.PHONY: srtool-build-wasm
srtool-build-wasm:
	@./scripts/build-wasm.sh

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

## benchmark for pallets

.PHONY: benchmark-frame-system
benchmark-frame-system:
	@./scripts/run-benchmark-pallet.sh frame_system

.PHONY: benchmark-account-linker
benchmark-account-linker:
	@./scripts/run-benchmark-pallet.sh account-linker

.PHONY: benchmark-offchain-worker
benchmark-offchain-worker:
	@./scripts/run-benchmark-pallet.sh pallet-offchain-worker

.PHONY: benchmark-nft
benchmark-nft:
	@./scripts/run-benchmark-pallet.sh pallet-nft

define pkgid
	$(shell cargo pkgid $1)
endef
