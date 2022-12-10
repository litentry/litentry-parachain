import { createERCDepositData, describeCrossChainTransfer } from './setup-bridge';
import { step } from 'mocha-steps';
import { toHex } from 'web3-utils';
import { signAndSend, sleep } from './utils';
import { assert } from 'chai';
import { BigNumber, ethers } from 'ethers';

const BN = require('bn.js');
const bn100e12 = new BN(10).pow(new BN(12)).mul(new BN(100));

describeCrossChainTransfer('Test Cross-chain Transfer', ``, (context) => {
    step('Transfer 100 Lit from eth to parachain', async function () {
        let bridge = context.ethConfig.bridge.connect(context.ethConfig.wallets.bob);
        let erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        // substrate native token
        // const destResourceId = "0x00000000000000000000000000000063a7e2be78898ba83824b0c0cc8dfb6001"
        const destResourceId = context.parachainConfig.api.consts.bridgeTransfer.nativeTokenResourceId.toHex();
        const depositAmount = toHex(BigNumber.from('100,000,000,000,000,000,000'.replace(/,/g, '')).toString());
        let destinationChainID = parseInt(context.parachainConfig.api.consts.chainBridge.bridgeChainId.toString());
        //FERDIE key command: polkadot key inspect //Ferdie
        const destinationRecipientAddress = '0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c';

        const beforeAccountData = await context.parachainConfig.api.query.system.account(
            context.parachainConfig.ferdie.address
        );
        console.log('before deposit: ', beforeAccountData.toString());

        // approve
        await erc20.approve(context.ethConfig.erc20Handler.address, depositAmount);
        await sleep(6);

        // deposit
        let data = createERCDepositData(depositAmount, 32, destinationRecipientAddress);
        await bridge.deposit(destinationChainID, destResourceId, data);
        await sleep(12 * 4);

        const afterAccountData = await context.parachainConfig.api.query.system.account(
            context.parachainConfig.ferdie.address
        );
        console.log('after deposit: ', afterAccountData.toString());

        assert.equal(
            bn100e12.add(beforeAccountData.data.free.toBn()).toString(),
            afterAccountData.data.free.toBn().toString()
        );
    });

    step('Transfer 100 Lit from parachain to eth', async function () {
        const receipt = context.ethConfig.wallets.charlie.address;
        let erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        const b: BigNumber = await erc20.balanceOf(receipt);
        await signAndSend(
            context.parachainConfig.api.tx.bridgeTransfer.transferNative(bn100e12.toString(), receipt, 0),
            context.parachainConfig.alice
        );
        // const fee = await context.parachainConfig.api.query.chainBridge.bridgeFee(0)
        await sleep(15);
        const actual_receive = BigNumber.from('99,000,000,000,000,000,000'.replace(/,/g, ''));
        assert.equal(b.add(actual_receive).toString(), (await erc20.balanceOf(receipt)).toString());
    });

    step('Boundary testing on ethereum: over the maximum balance', async function () {
        const receipt = context.ethConfig.wallets.charlie.address;
        const handlerBalance: BigNumber = await context.ethConfig.erc20.balanceOf(
            context.ethConfig.erc20Handler.address
        );
        const fee = await context.parachainConfig.api.query.chainBridge.bridgeFee(0);
        const Bridge = require('../bridge/contracts/Bridge.json');
        const inter = new ethers.utils.Interface(Bridge.abi);
        await signAndSend(
            context.parachainConfig.api.tx.bridgeTransfer.transferNative(
                handlerBalance
                    .div(BigNumber.from(1000000))
                    .add(BigNumber.from(100))
                    .add(BigNumber.from(fee.toString())).toString(),
                receipt,
                0
            ),
            context.parachainConfig.alice
        );
        const provider = context.ethConfig.wallets.alice.provider;
        const currentBlock = await provider.getBlockNumber();
        await sleep(15);
        for (let i = currentBlock; i <= (await provider.getBlockNumber()); i++) {
            const block = await provider.getBlockWithTransactions(i);
            for (let j = 0; j < block.transactions.length; j++) {
                if (block.transactions[j].to === context.ethConfig.bridge.address) {
                    const tx = block.transactions[j];
                    const decodedInput = inter.parseTransaction({ data: tx.data, value: tx.value });
                    if (decodedInput.name === 'executeProposal') {
                        const receipt = await provider.getTransactionReceipt(tx.hash);
                        assert.equal(0, receipt.status, 'Expect the transaction fail, it actually succeeds');
                        return;
                    }
                }
            }
        }
        assert.fail('could not find any transactions');
    });

    step('Boundary testing on ethereum: equal to the maximum balance', async function () {
        const receipt = context.ethConfig.wallets.charlie.address;
        const handlerBalance: BigNumber = await context.ethConfig.erc20.balanceOf(
            context.ethConfig.erc20Handler.address
        );
        const erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        const fee = await context.parachainConfig.api.query.chainBridge.bridgeFee(0);
        await signAndSend(
            context.parachainConfig.api.tx.bridgeTransfer.transferNative(
                handlerBalance.div(BigNumber.from(1000000)).add(BigNumber.from(fee.toString())).toString(),
                receipt,
                0
            ),
            context.parachainConfig.alice
        );
        await sleep(15);
        assert.equal((await erc20.balanceOf(context.ethConfig.erc20Handler.address)).toString(), '0');
    });

    step('Boundary testing on parachain', async function () {
        let bridge = context.ethConfig.bridge.connect(context.ethConfig.wallets.bob);
        let erc20 = context.ethConfig.erc20.connect(context.ethConfig.wallets.bob);
        // substrate native token
        const destResourceId = context.parachainConfig.api.consts.bridgeTransfer.nativeTokenResourceId.toHex();
        const total_issuance = (await context.parachainConfig.api.query.balances.totalIssuance()).toBn();
        const maximum_issuance = new BN(
            (await context.parachainConfig.api.query.bridgeTransfer.maximumIssuance()).toString()
        );
        await context.ethConfig.erc20.mint(
            context.ethConfig.wallets.bob.address,
            maximum_issuance.sub(new BN(1000)).mul(new BN(1000000)).toString()
        );
        const depositAmount = maximum_issuance.sub(total_issuance).add(new BN(1000)).mul(new BN(1000000));
        let destinationChainID = parseInt(context.parachainConfig.api.consts.chainBridge.bridgeChainId.toString());

        const destinationRecipientAddress = '0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c';
        await erc20.approve(context.ethConfig.erc20Handler.address, `0x${depositAmount.toString('hex')}`);
        await sleep(2);
        let data = createERCDepositData(`0x${depositAmount.toString('hex')}`, 32, destinationRecipientAddress);
        await bridge.deposit(destinationChainID, destResourceId, data);
        let expectResult = false;
        const block = await context.parachainConfig.api.rpc.chain.getBlock();
        const blockNumber = block.block.header.number;
        const unsubscribe = await context.parachainConfig.api.rpc.chain.subscribeNewHeads(async (header) => {
            console.log(`Chain is at block: #${header.number}`);
            const signedBlock = await context.parachainConfig.api.rpc.chain.getBlock(header.hash);
            const apiAt = await context.parachainConfig.api.at(signedBlock.block.header.hash);
            const allRecords = await apiAt.query.system.events();
            if (header.number.toNumber() > blockNumber.toNumber() + 4) {
                unsubscribe();
                assert.fail('expect the transaction fail in the last 4 blocks, but not found');
            }
            signedBlock.block.extrinsics.forEach((ex, index) => {
                if (!(ex.method.section === 'chainBridge' && ex.method.method === 'acknowledgeProposal')) {
                    return;
                }
                allRecords
                    .filter(({ phase }) => phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index))
                    .forEach(({ event }) => {
                        if (context.parachainConfig.api.events.system.ExtrinsicFailed.is(event)) {
                            const [dispatchError, dispatchInfo] = event.data;
                            let errorInfo;
                            // decode the error
                            if (dispatchError.isModule) {
                                // for module errors, we have the section indexed, lookup
                                // (For specific known errors, we can also do a check against the
                                // api.errors.<module>.<ErrorName>.is(dispatchError.asModule) guard)
                                const decoded = context.parachainConfig.api.registry.findMetaError(
                                    dispatchError.asModule
                                );
                                errorInfo = `${decoded.section}.${decoded.name}`;
                            } else {
                                // Other, CannotLookup, BadOrigin, no extra info
                                errorInfo = dispatchError.toString();
                            }
                            expectResult = true;
                            console.log(`chainBridge.acknowledgeProposal:: ExtrinsicFailed:: ${errorInfo}`);
                            return;
                        }
                    });
            });
            if (expectResult) {
                unsubscribe();
                assert.exists('');
            }
        });
        await sleep(39);
    });
});
