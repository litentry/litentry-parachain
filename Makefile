all:
	@make help
# variant declaration

NODE_BIN=litentry-collator
RUNTIME=litentry-parachain-runtime

.PHONY: help ## Display help commands.
help:
	@printf 'Usage:\n'
	@printf '  make <tagert>\n'
	@printf '\n'
	@printf 'Targets:\n'
	@IFS=$$'\n' ; \
    help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//'`); \
    for help_line in $${help_lines[@]}; do \
        IFS=$$'#' ; \
        help_split=($$help_line) ; \
        help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		IFS=$$':' ; \
		phony_command=($$help_split); \
        help_command=`echo $${phony_command[1]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		printf "  %-50s %s\n" $$help_command $$help_info ; \
    done

# build release

.PHONY: build-all ## Build release version.
build-all:
	cargo build --release

.PHONY: build-node ## Build release node only.
build-node:
	cargo build -p $(call pkgid, $(NODE_BIN)) --release

.PHONY: build-runtime ## Build release runtime only.
build-runtime:
	cargo build -p $(call pkgid, $(RUNTIME)) --release

# use srtool to build wasm locally
# in github actions srtool-actions is used
.PHONY: srtool-build-wasm
srtool-build-wasm:
	@./scripts/build-wasm.sh

.PHONY: build-docker ## Build docker image.
build-docker:
	@./scripts/build-docker.sh

.PHONY: build-spec-dev ## Build specifiction without bootnodes.
build-spec-dev:
	./target/release/$(NODE_BIN) build-spec --chain dev --disable-default-bootnode > ./source/local.json

.PHONY: build-benchmark ## Build release version with `runtime-benchmarks`
build-benchmark:
	cargo build --features runtime-benchmarks --release

# test

.PHONY: test-node
test-node:
	cargo test --package $(call pkgid, $(NODE_BIN))

.PHONY: test-ci
test-ci: launch-local-docker
	@./scripts/run-ci-test.sh

# format

.PHONY: format ## Format source code by `cargo fmt`.
format:
	cargo fmt --all -- --check

# launch a local dev network using docker
.PHONY: launch-local-docker ## Launch dev parachain by docker locally
launch-local-docker: generate-docker-compose-dev
	@cd docker/generated-dev; docker-compose up -d --build

# stop the local dev containers and cleanup images
# for the most part used when done with launch-local-docker
.PHONY: clean-local-docker ## Clean up docker images, containers, volumes, etc.
clean-local-docker:
	@./scripts/clean-local-docker.sh

# generate docker-compose files

.PHONY: generate-docker-compose-dev ## Generate dev docker-compose files.
generate-docker-compose-dev:
	@./scripts/generate-docker-files.sh dev

.PHONY: generate-docker-compose-staging ## Generate staging docker-compose files.
generate-docker-compose-staging:
	@./scripts/generate-docker-files.sh staging

# benchmark for pallets

.PHONY: benchmark-frame-system ## Benchmark pallet `frame-system`.
benchmark-frame-system:
	@./scripts/run-benchmark-pallet.sh frame_system

.PHONY: benchmark-account-linker  ## Benchmark pallet `account-linker`.
benchmark-account-linker:
	@./scripts/run-benchmark-pallet.sh account-linker

.PHONY: benchmark-offchain-worker ## Benchmark pallet `offchain-worker`.
benchmark-offchain-worker:
	@./scripts/run-benchmark-pallet.sh pallet-offchain-worker

.PHONY: benchmark-nft ## Benchmark pallet `nft`.
benchmark-nft:
	@./scripts/run-benchmark-pallet.sh pallet-nft

define pkgid
	$(shell cargo pkgid $1)
endef
