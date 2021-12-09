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

.PHONY: build-node ## Build release node with default features
build-node:
	cargo build -p $(call pkgid, $(NODE_BIN)) --release

.PHONY: build-runtime ## Build release runtime
build-runtime:
	cargo build -p $(call pkgid, $(RUNTIME)) --release

.PHONY: srtool-build-wasm ## Build wasm locally with srtools
srtool-build-wasm:
	@./scripts/build-wasm.sh

.PHONY: build-docker ## Build docker image
build-docker:
	@./scripts/build-docker.sh

.PHONY: build-node-benchmarks ## Build release version with `runtime-benchmarks` feature
build-node-benchmarks:
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

define pkgid
	$(shell cargo pkgid $1)
endef
