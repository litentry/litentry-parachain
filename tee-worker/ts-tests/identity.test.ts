import { describeLitentry, generateVerificationMessage, getMessage } from "./utils";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import {
    linkIdentity,
    setUserShieldingKey,
    unlinkIdentity,
    verifyIdentity,
} from "./indirect_calls";
import { step } from "mocha-steps";
import { assert } from "chai";
import { LitentryIdentity, LitentryValidationData } from "./type-definitions";
import { Sign } from "./web3/functions";
const twitterIdentity = <LitentryIdentity>{
    handle: {
        PlainString: `0x${Buffer.from("mock_user", "utf8").toString("hex")}`,
    },
    web_type: {
        Web2Identity: "Twitter",
    },
};

const twitterValidationData = <LitentryValidationData>{
    Web2Validation: {
        Twitter: {
            tweet_id: `0x${Buffer.from("100", "utf8").toString("hex")}`,
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

    step("set user shielding key", async function () {
        //get signature
        // const message = getMessage(context.defaultSigner.address, "polkadot-js");
        // const signature = await Sign(message, context.defaultSigner);

        const who = await setUserShieldingKey(context, context.defaultSigner, aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), "check caller error");
    });

    step("link twitter identity", async function () {
        const r = await linkIdentity(context, context.defaultSigner, aesKey, true, twitterIdentity);
        if (r) {
            const [_who, challengeCode] = r;
            console.log("challengeCode: ", challengeCode);
            const msg = generateVerificationMessage(
                context,
                hexToU8a(challengeCode),
                context.defaultSigner.addressRaw,
                twitterIdentity
            );
            console.log("post verification msg to twitter: ", msg);
            assert.isNotEmpty(challengeCode, "challengeCode empty");
        }
    });

    step("verify twitter identity", async function () {
        const who = await verifyIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity,
            twitterValidationData
        );
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), "check caller error");
    });

    step("unlink identity", async function () {
        const who = await unlinkIdentity(
            context,
            context.defaultSigner,
            aesKey,
            true,
            twitterIdentity
        );
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), "check caller error");
    });
});
