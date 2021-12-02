all:
	@make help
# variant declaration

NODE_BIN=litentry-collator
RUNTIME=litentry-parachain-runtime

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

.PHONY: build-all ## Build release version
build-all:
	cargo build --release

.PHONY: build-node ## Build release node only
build-node:
	cargo build -p $(call pkgid, $(NODE_BIN)) --release

.PHONY: build-runtime ## Build release runtime only
build-runtime:
	cargo build -p $(call pkgid, $(RUNTIME)) --release

.PHONY: srtool-build-wasm ## Build wasm locally with srtools
srtool-build-wasm:
	@./scripts/build-wasm.sh

.PHONY: build-docker ## Build docker image
build-docker:
	@./scripts/build-docker.sh

.PHONY: build-spec-dev ## Build specifiction without bootnodes
build-spec-dev:
	./target/release/$(NODE_BIN) build-spec --chain dev --disable-default-bootnode > ./source/local.json

.PHONY: build-benchmark ## Build release version with `runtime-benchmarks`
build-benchmark:
	cargo build --features runtime-benchmarks --release
	
# launching local dev networks

.PHONY: launch-local-docker ## Launch dev parachain network with docker locally
launch-local-docker: generate-docker-compose-dev
	@./scripts/launch-local-docker.sh

.PHONY: launch-local-binary ## Launch dev parachain network with binaries locally
launch-local-binary:
	@./scripts/launch-local-binary.sh

# run tests

.PHONY: test-node
test-node:
	cargo test --package $(call pkgid, $(NODE_BIN))

.PHONY: test-ci-docker ## Run CI tests with docker without clean-up
test-ci-docker: launch-local-docker
	@./scripts/run-ci-test.sh

.PHONY: test-ci-binary ## Run CI tests with binary without clean-up
test-ci-binary: launch-local-binary
	@./scripts/run-ci-test.sh

# clean up

.PHONY: clean-local-docker ## Clean up docker images, containers, volumes, etc
clean-local-docker:
	@./scripts/clean-local-docker.sh

.PHONY: clean-local-binary ## Clean up (kill) started relaychain and parachain binaries
clean-local-binary:
	@./scripts/clean-local-binary.sh

# generate docker-compose files

.PHONY: generate-docker-compose-dev ## Generate dev docker-compose files
generate-docker-compose-dev:
	@./scripts/generate-docker-files.sh dev

# format

.PHONY: fmtcheck ## cargo fmt check
fmtcheck:
	cargo fmt --all -- --check

.PHONY: fmt ## cargo fmt all
fmt:
	cargo fmt --all

# clippy

.PHONY: clippy ## cargo clippy
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# benchmark for pallets

.PHONY: benchmark-frame-system ## Benchmark pallet `frame-system`
benchmark-frame-system:
	@./scripts/run-benchmark-pallet-local.sh frame-system

.PHONY: benchmark-pallet-timestamp
benchmark-pallet-timestamp:
	@./scripts/run-benchmark-pallet-local.sh pallet-timestamp

.PHONY: benchmark-pallet-utility
benchmark-pallet-utility:
	@./scripts/run-benchmark-pallet-local.sh pallet-utility

.PHONY: benchmark-pallet-scheduler
benchmark-pallet-scheduler:
	@./scripts/run-benchmark-pallet-local.sh pallet-scheduler

.PHONY: benchmark-pallet-treasury
benchmark-pallet-treasury:
	@./scripts/run-benchmark-pallet-local.sh pallet-treasury

.PHONY: benchmark-pallet-democracy
benchmark-pallet-democracy:
	@./scripts/run-benchmark-pallet-local.sh pallet-democracy

.PHONY: benchmark-pallet-collective
benchmark-pallet-collective:
	@./scripts/run-benchmark-pallet-local.sh pallet-collective

.PHONY: benchmark-pallet-proxy
benchmark-pallet-proxy:
	@./scripts/run-benchmark-pallet-local.sh pallet-proxy

.PHONY: benchmark-pallet-balances
benchmark-pallet-balances:
	@./scripts/run-benchmark-pallet-local.sh pallet-balances

.PHONY: benchmark-pallet-membership
benchmark-pallet-membership:
	@./scripts/run-benchmark-pallet-local.sh pallet-membership

.PHONY: benchmark-pallet-collator-selection
benchmark-pallet-collator-selection:
	@./scripts/run-benchmark-pallet-local.sh pallet-collator-selection

.PHONY: benchmark-pallet-multisig
benchmark-pallet-multisig:
	@./scripts/run-benchmark-pallet-local.sh pallet-multisig

define pkgid
	$(shell cargo pkgid $1)
endef
