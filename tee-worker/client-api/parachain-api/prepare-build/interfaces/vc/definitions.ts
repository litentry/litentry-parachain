export default {
    types: {
        VCRequested: {
            account: "AccountId",
            mrEnclave: "ShardIdentifier",
            assertion: "Assertion",
        },
        Assertion: {
            _enum: {
                A1: "Null",
                A2: "Bytes",
                A3: "(Bytes,Bytes,Bytes)",
                A4: "Bytes",
                A6: "Null",
                A7: "Bytes",
                A8: "Vec<AssertionSupportedNetwork>",
                A10: "Bytes",
                A11: "Bytes",
                A12: "Bytes",
                A13: "AccountId32",
                A14: "Null",
                GenericDiscordRole: "GenericDiscordRoleType",
            },
        },
        AssertionSupportedNetwork: {
            _enum: ["Litentry", "Litmus", "LitentryRococo", "Polkadot", "Kusama", "Khala", "Ethereum", "TestNet"],
        },
        RequestVCResult: {
            vc_index: "H256",
            vc_hash: "H256",
            vc_payload: "AesOutput",
        },
        GenericDiscordRoleType: {
            _enum: {
                Contest: "GenericDiscordRoleContestType",
                SoraQuiz: "GenericDiscordRoleSoraQuizType",
            },
        },
        GenericDiscordRoleContestType: {
            _enum: ["Legend", "Popularity", "Participant"],
        },
        GenericDiscordRoleSoraQuizType: {
            _enum: ["Attendee", "Master"],
        },
    },
};
