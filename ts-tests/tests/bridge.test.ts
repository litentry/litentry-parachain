import {createERCDepositData, describeCrossChainTransfer} from "./setup-bridge";
import {step} from "mocha-steps";
import {toHex} from 'web3-utils';
import {signAndSend, sleep} from "./utils";
import {assert} from 'chai';
import {BigNumber} from "ethers";

const BN = require('bn.js');
const bn100e12 = new BN(10).pow(new BN(12)).mul(new BN(100));

describeCrossChainTransfer('Test Cross-chain Transfer', ``, (context) => {
    step('Transfer 100 Lit from eth to parachain', async function () {
        let bridge = context.ethConfig.bridge.connect(context.ethConfig.wallets.bob);
        let erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        // substrate native token
        // const destResourceId = "0x00000000000000000000000000000063a7e2be78898ba83824b0c0cc8dfb6001"
        const destResourceId = context.parachainConfig.api.consts.bridgeTransfer.nativeTokenResourceId.toHex()
        const depositAmount = toHex(BigNumber.from('100,000,000,000,000,000,000'.replace(/,/g, "")).toString());
        let destinationChainID = 1;
        //FERDIE key command: polkadot key inspect //Ferdie
        const destinationRecipientAddress = "0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c";

        await erc20.approve(context.ethConfig.erc20Handler.address, depositAmount)
        await sleep(2)
        let data = createERCDepositData(
            depositAmount,
            32,
            destinationRecipientAddress);
        const beforeAccountData = await context.parachainConfig.api.query.system.account(context.parachainConfig.ferdie.address);
        await bridge.deposit(destinationChainID, destResourceId, data)
        await sleep(36);
        const afterAccountData = await context.parachainConfig.api.query.system.account(context.parachainConfig.ferdie.address);
        assert.equal(bn100e12.add(beforeAccountData.data.free.toBn()).toString(), afterAccountData.data.free.toBn().toString())
    });


    step("Transfer 100 Lit from parachain to eth", async function () {
        const receipt = context.ethConfig.wallets.charlie.address;
        let erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        const b: BigNumber = await erc20.balanceOf(receipt)
        await signAndSend(context.parachainConfig.api.tx.bridgeTransfer.transferNative(bn100e12.toString(), receipt, 0), context.parachainConfig.alice)
        // const fee = await context.parachainConfig.api.query.chainBridge.bridgeFee(0)
        await sleep(15)
        const actual_receive = BigNumber.from('99,000,000,000,000,000,000'.replace(/,/g, ""))
        assert.equal(b.add(actual_receive).toString(), (await erc20.balanceOf(receipt)).toString())
    })
})