SHELL=/bin/bash
all:
	@make help

# variant declaration

NODE_BIN=litentry-collator

.PHONY: help ## Display help commands
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

.PHONY: build-all ## Build release all
build-all:
	cargo build --locked --release

.PHONY: build-node ## Build release node
build-node:
	cargo build --locked -p $(call pkgid, $(NODE_BIN)) --release

.PHONY: build-runtime-litentry ## Build litentry release runtime
build-runtime-litentry:
	cargo build --locked -p $(call pkgid, litentry-parachain-runtime) --release

.PHONY: build-runtime-rococo ## Build rococo release runtime
build-runtime-rococo:
	cargo build --locked -p $(call pkgid, rococo-parachain-runtime) --release

.PHONY: build-docker-release ## Build docker image using cargo profile `release`
build-docker-release:
	@./scripts/build-docker.sh release latest

.PHONY: build-docker-production ## Build docker image using cargo profile `production`
build-docker-production:
	@./scripts/build-docker.sh production

.PHONY: build-node-benchmarks ## Build release node with `runtime-benchmarks` feature
build-node-benchmarks:
	cargo build --locked --features runtime-benchmarks --release

.PHONY: build-node-tryruntime ## Build release node with `try-runtime` feature
build-node-tryruntime:
	cargo build --locked --features try-runtime --release

# launch a local network

.PHONY: launch-standalone ## Launch a local standalone node without relaychain network
launch-standalone:
	@./scripts/launch-standalone.sh

.PHONY: launch-network-rococo ## Launch a local rococo network with relaychain network
launch-network-rococo:
	@./scripts/launch-network.sh rococo

.PHONY: launch-network-litentry ## Launch a local litentry network with relaychain network
launch-network-litentry:
	@./scripts/launch-network.sh litentry

.PHONY: launch-docker-bridge
launch-docker-bridge:
	@./scripts/launch-local-bridge-docker.sh

# run tests

.PHONY: test-cargo-all ## cargo test --all
test-cargo-all:
	@cargo test --release --all

.PHONY: test-cargo-all-benchmarks ## cargo test --all --features runtime-benchmarks
test-cargo-all-benchmarks:
	@cargo test --release --all --features runtime-benchmarks

.PHONY: test-ts-litentry ## Run litentry ts tests without clean-up
test-ts-litentry: launch-network-litentry
	@./scripts/run-ts-test.sh litentry bridge evm

.PHONY: test-ts-rococo ## Run rococo ts tests without clean-up
test-ts-rococo: launch-network-rococo
	@./scripts/run-ts-test.sh rococo bridge evm


# clean up

.PHONY: clean-network ## Clean up the network launched by 'launch-network'
clean-network:
	@./scripts/clean-network.sh

# update dependencies

.PHONY: update-ts-dep ## update ts-tests dependencies
update-ts-dep:
	@cd ts-tests && pnpm dlx npm-check-updates -u && pnpm install

# format

.PHONY: fmt ## (cargo, taplo, ts, solidity) fmt
fmt: fmt-cargo fmt-taplo fmt-ts fmt-contract

.PHONY: fmt-cargo ## cargo fmt
fmt-cargo:
	@cargo fmt --all
	@cd tee-worker && cargo fmt --all
	@cd tee-worker/enclave-runtime && cargo fmt --all
	@cd bitacross-worker && cargo fmt --all
	@cd bitacross-worker/enclave-runtime && cargo fmt --all

.PHONY: fmt-taplo ## taplo fmt
fmt-taplo:
	@RUST_LOG=error taplo fmt
	@cd tee-worker && RUST_LOG=error taplo fmt
	@cd tee-worker/enclave-runtime && RUST_LOG=error taplo fmt

.PHONY: fmt-ts ## ts fmt
fmt-ts:
	@cd ts-tests && pnpm install && pnpm run format
	@cd tee-worker/ts-tests && pnpm install && pnpm run format

.PHONY: fmt-contract ## contract fmt
fmt-contract:
	@cd tee-worker/litentry/core/assertion-build/src/dynamic && pnpm install && pnpm run format

.PHONY: githooks ## install the githooks
githooks:
	git config core.hooksPath .githooks

# clippy

.PHONY: clippy ## cargo clippy
clippy:
	SKIP_WASM_BUILD=1 cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: clippyfix ## cargo clippy --fix
clippyfix:
	SKIP_WASM_BUILD=1 cargo clippy --allow-dirty --allow-staged --fix --workspace --all-targets --all-features -- -D warnings

.PHONY: cargofix ## cargo fix
cargofix:
	cargo fix --allow-dirty --allow-staged --workspace --all-targets --all-features

# cargo update

.PHONY: update ## cargo update
update:
	cargo update
	cd tee-worker && cargo update
	cd tee-worker/enclave-runtime && cargo update

# shellcheck

.PHONY: shellcheck ## check the shell scripts with WARNING level
shellcheck:
	@set -e
	@echo "checking parachain scripts..."
	@find scripts -name "*.sh" | xargs shellcheck -S warning
	@echo "checking tee-worker scripts..."
	@find tee-worker/scripts/litentry/ -name "*.sh" | xargs shellcheck -S warning
	@echo "Ok"

define pkgid
$(shell cargo pkgid $1)
endef
