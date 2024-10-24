# Copyright (C) 2023 Gramine contributors
# SPDX-License-Identifier: BSD-3-Clause

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

SELF_EXE = target/release/omni-executor

.PHONY: all
all: $(SELF_EXE) omni-executor.manifest
ifeq ($(SGX),1)
all: omni-executor.manifest.sgx omni-executor.sig
endif

ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = debug
else
GRAMINE_LOG_LEVEL = error
endif

# Note that we're compiling in release mode regardless of the DEBUG setting passed
# to Make, as compiling in debug mode results in an order of magnitude's difference in
# performance that makes testing by running a benchmark with ab painful. The primary goal
# of the DEBUG setting is to control Gramine's loglevel.
-include $(SELF_EXE).d # See also: .cargo/config.toml
$(SELF_EXE): Cargo.toml
	cargo build --release

omni-executor.manifest: omni-executor.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dself_exe=$(SELF_EXE) \
		$< $@

# Make on Ubuntu <= 20.04 doesn't support "Rules with Grouped Targets" (`&:`),
# see the helloworld example for details on this workaround.
omni-executor.manifest.sgx tee-bridge.sig: sgx_sign
	@:

.INTERMEDIATE: sgx_sign
sgx_sign: omni-executor.manifest $(SELF_EXE)
	gramine-sgx-sign \
		--manifest $< \
		--output $<.sgx

ifeq ($(SGX),)
GRAMINE = gramine-direct
else
GRAMINE = gramine-sgx
endif

.PHONY: start-gramine-server
start-gramine-server: all
	$(GRAMINE) tee-bridge

.PHONY: clean
clean:
	$(RM) -rf *.token *.sig *.manifest.sgx *.manifest result-* OUTPUT

.PHONY: distclean
distclean: clean
	$(RM) -rf target/ Cargo.lock

.PHONY: build-docker
build-docker:
	docker build . --tag omni-executor:latest

.PHONY: start-local
start-local:
	docker-compose up

.PHONY: stop-local
stop-local:
	docker-compose down

.PHONY: get-omni-pallet-metadata
get-omni-pallet-metadata:
	subxt metadata --url http://localhost:9944 --allow-insecure --pallets OmniAccount > parentchain/artifacts/rococo-omni-account.scale
