import {
    describeLitentry,
    generateVerificationMessage,
    getMessage,
    listenEncryptedEvents,
} from "./utils";
import { hexToU8a, u8aToHex, stringToU8a } from "@polkadot/util";
import {
    createIdentity,
    setUserShieldingKey,
    removeIdentity,
    verifyIdentity,
} from "./indirect_calls";
import { step } from "mocha-steps";
import { assert } from "chai";
import {IdentityGenericEvent, LitentryIdentity, LitentryValidationData} from "./type-definitions";
import { Sign } from "./web3/functions";
import { generateTestKeys } from "./web3/functions";
import { ethers } from "ethers";
import { HexString } from "@polkadot/util/types";
import {KeyringPair} from "@polkadot/keyring/types";
const twitterIdentity = <LitentryIdentity>{
    handle: {
        PlainString: `0x${Buffer.from("mock_user", "utf8").toString("hex")}`,
    },
    web_type: {
        Web2Identity: "Twitter",
    },
};

const ethereumIdentity = <LitentryIdentity>{
    handle: {
        Address20: `0xff93B45308FD417dF303D6515aB04D9e89a750Ca`,
    },
    web_type: {
        Web3Identity: {
            Evm: "Ethereum",
        },
    },
};

const substrateIdentity = <LitentryIdentity>{
    handle: {
        Address32: `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`, //alice
    },
    web_type: {
        Web3Identity: {
            Substrate: "Litentry",
        },
    },
};
const twitterValidationData = <LitentryValidationData>{
    Web2Validation: {
        Twitter: {
            tweet_id: `0x${Buffer.from("100", "utf8").toString("hex")}`,
        },
    },
};

const ethereumValidationData = <LitentryValidationData>{
    Web3Validation: {
        Evm: {
            message: `0x${Buffer.from("mock_message", "utf8").toString("hex")}`,
            signature: {
                Ethereum: `0x${Buffer.from(
                    "10ee76e356d944d17bce552a4fd0d4554ccc97dc81213f470367bd3b99c441c51",
                    "utf8"
                ).toString("hex")}`,
            },
        },
    },
};
const substrateValidationData = <LitentryValidationData>{
    Web3Validation: {
        Substrate: {
            message: `0x${Buffer.from("mock_message", "utf8").toString("hex")}`,
            signature: {
                Sr25519: `0x${Buffer.from(
                    "10ee76e356d944d17bce552a4fd0d4554ccc97dc81213f470367bd3b99c441c51",
                    "utf8"
                ).toString("hex")}`,
            },
        },
    },
};
const discordIdentity = <LitentryIdentity>{
    handle: {
        PlainString: `0x${Buffer.from("859641379851337798", "utf8").toString("hex")}`,
    },
    web_type: {
        Web2Identity: "Discord",
    },
};

const discordValidationData = <LitentryValidationData>{
    Web2Validation: {
        Discord: {
            channel_id: `0x${Buffer.from("919848392035794945", "utf8").toString("hex")}`,
            guild_id: `0x${Buffer.from("919848390156767232", "utf8").toString("hex")}`,
            message_id: `0x${Buffer.from("859641379851337798", "utf8").toString("hex")}`,
        },
    },
};

describeLitentry("Test Identity", (context) => {
    const aesKey = "0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12";
    var signature_ethereum;
    var signature_substrate;

    step("set user shielding key", async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner, aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), "check caller error");
    });

    step("create identity", async function () {
        //create twitter identity
        const resp_twitter = await createIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity
        );
        if (resp_twitter) {
            const [_who, challengeCode] = resp_twitter;
            console.log("twitterIdentity challengeCode: ", challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(challengeCode),
                context.defaultSigner.addressRaw,
                twitterIdentity
            );
            console.log("post verification msg to twitter: ", msg);
            assert.isNotEmpty(challengeCode, "challengeCode empty");
        }
        //create ethereum identity
        const resp_ethereum = await createIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            ethereumIdentity
        );
        if (resp_ethereum) {
            const [_who, challengeCode] = resp_ethereum;
            console.log("ethereumIdentity challengeCode: ", challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(challengeCode),
                context.defaultSigner.addressRaw,
                ethereumIdentity
            );
            console.log("post verification msg to ethereum: ", msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = await context.ethersWallet.alice.signMessage(msgHash);
            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            assert.isNotEmpty(challengeCode, "challengeCode empty");
        }
        // create substrate identity
        const resp_substrate = await createIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity
        );
        if (resp_substrate) {
            const [_who, challengeCode] = resp_substrate;
            console.log("substrateIdentity challengeCode: ", challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(challengeCode),
                context.defaultSigner.addressRaw,
                substrateIdentity
            );

            console.log("post verification msg to substrate: ", msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = context.defaultSigner.sign(msg);
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 =
                u8aToHex(signature_substrate);
            assert.isNotEmpty(challengeCode, "challengeCode empty");
        }
    });

    step("verify identity", async function () {
        //verify twitter identity
        const twitter_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity,
            twitterValidationData
        );
        assertIdentityVerified(context.defaultSigner, twitter_identity_verified);

        // verify ethereum identity
        const ethereum_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            ethereumIdentity,
            ethereumValidationData
        );
        assertIdentityVerified(context.defaultSigner, ethereum_identity_verified);

        //verify substrate identity
        const substrate_identity_verified = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity,
            substrateValidationData
        );
        assertIdentityVerified(context.defaultSigner, substrate_identity_verified);
    });

    step("remove identity", async function () {
        //remove twitter identity
        const twitter_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity
        );
        assertIdentityRemoved(context.defaultSigner, twitter_identity_removed)

        // remove ethereum identity
        const ethereum_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            ethereumIdentity
        );
        assertIdentityRemoved(context.defaultSigner, ethereum_identity_removed)

        // remove substrate identity
        const substrate_identity_removed = await removeIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            substrateIdentity
        );
        assertIdentityRemoved(context.defaultSigner, substrate_identity_removed)
    });
});


function assertIdentityVerified(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false
    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true;
                assert.isTrue(identityEvent.idGraph[i][1].is_verified, "identity should be verified");
            }
        }
    }
    assert.isTrue(idGraphExist, "id_graph should exist")
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), "check caller error");
}

function assertIdentityRemoved(signer: KeyringPair, identityEvent: IdentityGenericEvent | undefined) {
    let idGraphExist = false
    if (identityEvent) {
        for (let i = 0; i < identityEvent.idGraph.length; i++) {
            if (JSON.stringify(identityEvent.idGraph[i][0]) == JSON.stringify(identityEvent.identity)) {
                idGraphExist = true
            }
        }
    }
    assert.isFalse(idGraphExist, "id_graph should be empty")
    assert.equal(identityEvent?.who, u8aToHex(signer.addressRaw), "check caller error");
}
