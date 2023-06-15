## Summary

### Changes

<!-- What changes are being made? Is this a change a bugfix or new functionality?  -->

### Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing client application to not work as expected)
- [ ] Documentation update
- [ ] Examples (adding tests)

## Why

<!-- Why are these changes needed? A link to the issue may be sufficient -->

## How (Optional)

<!-- How were these changes implemented? -->

## Breaking change checklist

<!-- Have you done all of these things?  -->
<!-- to check an item, place an "x" in the box like so: "- [x] Automated tests" -->

- [ ] If any of below files is changed, pls add the breaking-change label to this PR, choose from C0-breaking, C1-noteworthy

1 PalletIdentityManagementCall
- [ ] pallets/identity-management/src/lib.rs
- [ ] tee-worker/litentry/pallets/identity-management/src/lib.rs

1.1 setShieldingKey
- [ ] pallets/identity-management/src/lib.rs#set_user_shielding_key
- [ ] pallets/identity-management/src/lib.rs#user_shielding_key_set
- [ ] tee-worker/litentry/pallets/identity-management/src/lib.rs#set_user_shielding_key

1.2 createIdentity

- [ ] pallets/identity-management/src/lib.rs#create_identity
- [ ] pallets/identity-management/src/lib.rs#identity_created
- [ ] pallets/tee-worker/litentry/pallets/identity-management/src/lib.rs#create_identity
- [ ] pallets/ tee-worker/app-libs/stf/src/trusted_call_litentry.rs#create_identity_internal

1.3 verifyIdentity
- [ ] pallets/identity-management/src/lib.rs#verify_identity
- [ ] pallets/identity-management/src/lib.rs#identity_verified
- [ ] pallets/tee-worker/litentry/pallets/identity-management/src/lib.rs#verify_identity

2 PalletVCManagementCall
- [ ] pallets/vc-management/src/lib.rs
- [ ] tee-worker/litentry/core/identity-verification
- [ ] tee-worker/litentry/core/assertion-build/src

2.1 Verify Credential
- [ ] tee-worker/litentry/core/credentials/src/lib.rs
- [ ] tee-worker/litentry/core/credentials/src/templates/credential_schema.json
- [ ] tee-worker/litentry/core/credentials/src/templates/credential.json

2.2 Request VC
- [ ] pallets/vc-management/src/lib.rs#request_vc
- [ ] pallets/vc-management/src/lib.rs# vc_issued


## Any Testing Evidences

- Please attach any relevent evidences


