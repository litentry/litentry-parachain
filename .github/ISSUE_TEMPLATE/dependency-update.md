---
name: Dependency update issue template
about: Tracking issue for new polkadot/substrate dependency updates
title: Update upstream dependency to <polkadot-version>
---

This is an issue to track the update of the upstream dependency.

### parachain dependency

- [ ] Update local polkadot/substrate dependencies (helper script: `./scripts/bump-code-versions.sh`)
- [ ] Fix any compilation/test issues
- [ ] Optionally roll out the client/runtime update

### tee-worker dependency

- [ ] Upstream PR: update [integritee-pallets](https://github.com/integritee-network/pallets) dependency
- [ ] Upstream PR: update [integritee-node](https://github.com/integritee-network/integritee-node) dependency
- [ ] Upstream PR: update [integritee-worker](https://github.com/integritee-network/worker) dependency
- [ ] Ask integritee to update [frontier](https://github.com/integritee-network/frontier) and [substrate-api-client](https://github.com/scs/substrate-api-client) dependency
- [ ] Merge upstream changes to `tee-worker/`
