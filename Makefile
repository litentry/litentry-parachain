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

.PHONY: build-all ## Build release version
build-all:
	cargo build --locked --release

.PHONY: build-node ## Build release node with default features
build-node:
	cargo build --locked -p $(call pkgid, $(NODE_BIN)) --release

.PHONY: build-runtime-litentry ## Build litentry release runtime
build-runtime-litentry:
	cargo build --locked -p $(call pkgid, litentry-parachain-runtime) --release

.PHONY: build-runtime-litmus ## Build litmus release runtime
build-runtime-litmus:
	cargo build --locked -p $(call pkgid, litmus-parachain-runtime) --release

.PHONY: build-runtime-rococo ## Build rococo release runtime
build-runtime-rococo:
	cargo build --locked -p $(call pkgid, rococo-parachain-runtime) --release

.PHONY: build-runtime-moonbase ## Build moonbase release runtime
build-runtime-moonbase:
	cargo build --locked -p $(call pkgid, moonbase-parachain-runtime) --release

.PHONY: srtool-build-wasm-litentry ## Build litentry wasm with srtools
srtool-build-wasm-litentry:
	@./scripts/build-wasm.sh litentry

.PHONY: srtool-build-wasm-litmus ## Build litmus wasm with srtools
srtool-build-wasm-litmus:
	@./scripts/build-wasm.sh litmus

.PHONY: srtool-build-wasm-rococo ## Build rococo wasm with srtools
srtool-build-wasm-rococo:
	@./scripts/build-wasm.sh rococo

.PHONY: srtool-build-wasm-moonbase ## Build moonbase wasm with srtools
srtool-build-wasm-moonbase:
	@./scripts/build-wasm.sh moonbase

.PHONY: build-docker-release ## Build docker image using cargo profile `release`
build-docker-release:
	@./scripts/build-docker.sh release

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

.PHONY: launch-docker-litentry ## Launch a local litentry-parachain network with docker
launch-docker-litentry: generate-docker-compose-litentry
	@./scripts/launch-local-docker.sh litentry

.PHONY: launch-docker-litmus ## Launch a local litmus-parachain network with docker
launch-docker-litmus: generate-docker-compose-litmus
	@./scripts/launch-local-docker.sh litmus

.PHONY: launch-binary-litentry ## Launch a local litentry-parachain network with binaries
launch-binary-litentry:
	@./scripts/launch-local-binary.sh litentry

.PHONY: launch-binary-litmus ## Launch a local litmus-parachain network with binaries
launch-binary-litmus:
	@./scripts/launch-local-binary.sh litmus

.PHONY: launch-binary-rococo ## Launch a local rococo-parachain network with binaries
launch-binary-rococo:
	@./scripts/launch-local-binary.sh rococo

# run tests

.PHONY: test-cargo-all ## cargo test --all
test-cargo-all:
	@cargo test --release --all

.PHONY: test-ts-docker-litentry ## Run litentry ts tests with docker without clean-up
test-ts-docker-litentry: launch-docker-litentry
	@./scripts/run-ts-test.sh

.PHONY: test-ts-docker-litmus ## Run litmus ts tests with docker without clean-up
test-ts-docker-litmus: launch-docker-litmus
	@./scripts/run-ts-test.sh

.PHONY: test-ts-binary-litentry ## Run litentry ts tests with binary without clean-up
test-ts-binary-litentry: launch-binary-litentry
	@./scripts/run-ts-test.sh

.PHONY: test-ts-binary-litmus ## Run litmus ts tests with binary without clean-up
test-ts-binary-litmus: launch-binary-litmus
	@./scripts/run-ts-test.sh

test-ts-binary-rococo: launch-binary-rococo
	@./scripts/launch-local-bridge-binary.sh
	@./scripts/run-ts-test.sh

# clean up

.PHONY: clean-docker-litentry ## Clean up litentry docker images, containers, volumes, etc
clean-docker-litentry:
	@./scripts/clean-local-docker.sh litentry

.PHONY: clean-docker-litmus ## Clean up litmus docker images, containers, volumes, etc
clean-docker-litmus:
	@./scripts/clean-local-docker.sh litmus

.PHONY: clean-binary ## Kill started polkadot and litentry-collator binaries
clean-binary:
	@./scripts/clean-local-binary.sh

# generate docker-compose files

.PHONY: generate-docker-compose-litentry ## Generate docker-compose files for litentry local network
generate-docker-compose-litentry:
	@./scripts/generate-docker-files.sh litentry

.PHONY: generate-docker-compose-litmus ## Generate docker-compose files for litmus local network
generate-docker-compose-litmus:
	@./scripts/generate-docker-files.sh litmus

# update dependencies

.PHONY: update-ts-dep ## update ts-tests dependencies
update-ts-dep:
	@cd ts-tests && npx npm-check-updates -u && yarn

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
	SKIP_WASM_BUILD=1 cargo clippy --workspace --all-targets --all-features -- -D warnings

define pkgid
	$(shell cargo pkgid $1)
endef
