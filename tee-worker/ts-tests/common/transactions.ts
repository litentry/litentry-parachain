import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { IntegrationTestContext, TransactionSubmit, RequestEvent } from './type-definitions';
import { KeyringPair } from '@polkadot/keyring/types';
import { getListenTimeoutInBlocks } from './utils';
import { EventRecord, Event } from '@polkadot/types/interfaces';
import { expect } from 'chai';
import colors from 'colors';

//transactions utils
export async function sendTxUntilInBlock(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>, signer: KeyringPair) {
    return new Promise<SubmittableResult>(async (resolve, reject) => {
        const nonce = await api.rpc.system.accountNextIndex(signer.address);
        await tx.signAndSend(signer, { nonce }, (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve(result);
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });
}

export async function sendTxUntilInBlockList(
    api: ApiPromise,
    txs: TransactionSubmit[],
    signer: KeyringPair | KeyringPair[]
) {
    const signers = Array.isArray(signer) ? signer : [signer];
    return Promise.all(
        txs.map(async ({ tx, nonce }, index) => {
            const s = signers[index % signers.length];
            // The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
            await tx.paymentInfo(s);
            const result = await new Promise((resolve, reject) => {
                tx.signAndSend(s, { nonce }, (result) => {
                    if (result.status.isInBlock) {
                        //catch error
                        if (result.dispatchError) {
                            if (result.dispatchError.isModule) {
                                const decoded = api.registry.findMetaError(result.dispatchError.asModule);
                                const { docs, name, section } = decoded;

                                console.log(`${section}.${name}: ${docs.join(' ')}`);
                                resolve(`${section}.${name}`);
                            } else {
                                console.log(result.dispatchError.toString());
                                resolve(result.dispatchError.toString());
                            }
                        } else {
                            console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                            resolve(result);
                        }
                    } else if (result.status.isInvalid) {
                        reject(`Transaction is ${result.status}`);
                    }
                });
            });

            return result;
        })
    );
}

export async function sendTxsWithUtility(
    context: IntegrationTestContext,
    signer: KeyringPair,
    txs: TransactionSubmit[],
    pallet: string,
    events: string[]
): Promise<string[] | Event[]> {
    //ensure the tx is in block
    const isInBlockPromise = new Promise((resolve, reject) => {
        context.api.tx.utility.batchAll(txs.map(({ tx }) => tx)).signAndSend(signer, async (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve({
                    block: result.status.asInBlock.toString(),
                });
            } else if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });

    await isInBlockPromise;
    const resp_events = await listenEvent(context.api, pallet, events, txs.length);
    expect(resp_events.length).to.be.equal(txs.length);
    return resp_events;
}

export async function multiAccountTxSender(
    context: IntegrationTestContext,
    txs: TransactionSubmit[],
    signers: KeyringPair[],
    pallet: string,
    events: string[]
): Promise<Event[]> {
    await sendTxUntilInBlockList(context.api, txs, signers);
    const resp_events = await listenEvent(context.api, pallet, events, txs.length);
    expect(resp_events.length).to.be.equal(txs.length);
    return resp_events;
}

// Subscribe to the chain until we get the first specified event with given `section` and `methods`.
// We can listen to multiple `methods` as long as they are emitted in the same block.
// The event consumer should do the decryption optionaly as it's event specific
export async function listenEvent(api: ApiPromise, section: string, methods: string[], txsLength: number) {
    return new Promise<Event[]>(async (resolve, reject) => {
        let startBlock = 0;
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            const timeout = await getListenTimeoutInBlocks(api);
            if (currentBlockNumber > startBlock + timeout) {
                reject('Timeout: No event received, please check the worker logs for more details');
                return;
            }
            console.log(`\n--------- block #${header.number}, hash ${header.hash} ---------\n`);
            const [signedBlock, apiAt] = await Promise.all([api.rpc.chain.getBlock(header.hash), api.at(header.hash)]);

            const records: EventRecord[] = (await apiAt.query.system.events()) as any;

            const filtered_events: EventRecord[] = [];
            signedBlock.block.extrinsics.forEach((extrinsic, index) => {
                records.forEach((e, i) => {
                    const s = e.event.section;
                    const m = e.event.method;
                    const d = e.event.data;

                    section === s && e.phase.asApplyExtrinsic.eq(index)
                        ? console.log(colors.green(`Event[${i}]: ${s}.${m} ${d}`))
                        : console.log(`Event[${i}]: ${s}.${m} ${d}`);
                });
                const events_in_extrinsic = records.filter(({ event, phase }) => {
                    if (
                        phase.isApplyExtrinsic &&
                        section === event.section &&
                        !methods.includes(event.method) &&
                        !(event.method in RequestEvent)
                    ) {
                        reject(`Expect event ${methods} but received unexpected event ${event.method}`);
                    }
                    return (
                        phase.isApplyExtrinsic &&
                        phase.asApplyExtrinsic.eq(index) &&
                        section === event.section &&
                        methods.includes(event.method)
                    );
                });
                events_in_extrinsic.forEach((event) => {
                    filtered_events.push(event);
                });
            });

            if (filtered_events.length === txsLength) {
                resolve(filtered_events.map((e) => e.event));
                unsubscribe();
                return;
            }
        });
    });
}
