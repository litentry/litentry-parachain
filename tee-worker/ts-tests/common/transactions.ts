import { ApiPromise } from '@polkadot/api';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { IntegrationTestContext, TransactionSubmit } from './type-definitions';
import { KeyringPair } from '@polkadot/keyring/types';
import { getListenTimeoutInBlocks } from './utils';
import { EventRecord, Event } from '@polkadot/types/interfaces';
import { HexString } from '@polkadot/util/types';
import { u8aToHex } from '@polkadot/util';
import { expect } from 'chai';
import colors from 'colors';
//transactions utils
export async function sendTxUntilInBlock(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>, signer: KeyringPair) {
    return new Promise<{ block: string; txHash: string }>(async (resolve, reject) => {
        const nonce = await api.rpc.system.accountNextIndex(signer.address);
        await tx.signAndSend(signer, { nonce }, (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve({
                    block: result.status.asInBlock.toString(),
                    txHash: result.txHash.toHex(),
                });
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
                            resolve({
                                block: result.status.asInBlock.toString(),
                                txHash: result.status.hash.toString(),
                            });
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

// Subscribe to the chain until we get the first specified event with given `section` and `methods`.
// We can listen to multiple `methods` as long as they are emitted in the same block.
// The event consumer should do the decryption optionaly as it's event specific
export async function listenEvent(
    api: ApiPromise,
    section: string,
    methods: string[],
    txsLength: number,
    signers: HexString[]
) {
    return new Promise<Event[]>(async (resolve, reject) => {
        let startBlock = 0;
        let events: EventRecord[] = [];
        const signerToIndexMap: Record<string, number> = {};
        for (let i = 0; i < signers.length; i++) {
            signerToIndexMap[signers[i]] = i;
        }
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            const timeout = await getListenTimeoutInBlocks(api);
            if (currentBlockNumber > startBlock + timeout) {
                reject('timeout');
                return;
            }
            console.log(`\n--------- block #${header.number}, hash ${header.hash} ---------\n`);
            const apiAt = await api.at(header.hash);

            const records: EventRecord[] = (await apiAt.query.system.events()) as any;
            records.forEach((e, i) => {
                const s = e.event.section;
                const m = e.event.method;
                const d = e.event.data;
                section === s
                    ? console.log(colors.green(`Event[${i}]: ${s}.${m} ${d}`))
                    : console.log(`Event[${i}]: ${s}.${m} ${d}`);
            });

            const filtered_events = records.filter(({ phase, event }) => {
                return phase.isApplyExtrinsic && section === event.section && methods.includes(event.method);
            });

            //We're going to have to filter by signer, because multiple txs is going to mix
            const filtered_events_with_signer = filtered_events
                .filter((event) => {
                    const signerDatas = event.event.data.find((d) => {
                        if (Array.isArray(d)) {
                            return d.find((v) => signers.includes(v.toHex()));
                        } else {
                            return signers.includes(d.toHex());
                        }
                    });
                    return !!signerDatas;
                })
                .sort((a, b) => {
                    //We need sort by signers order
                    //First convert the signers array into an object signerToIndexMap, where the keys are each element in the signers array and the values are the index of that element in the array.
                    //Then, for each of the filtered events that match the given section and methods, the function uses the find function to locate the index of a specific parameter in the signers array.
                    //Then, it sorts the events based on this index so that the resulting event array is sorted according to the order of the signers array.
                    const signerIndexA =
                        signerToIndexMap[
                            a.event.data
                                .find((d) => {
                                    if (Array.isArray(d)) {
                                        return d.find((v) => signers.includes(v.toHex()));
                                    } else {
                                        return signers.includes(d.toHex());
                                    }
                                })!
                                .toHex()
                        ];
                    const signerIndexB =
                        signerToIndexMap[
                            b.event.data
                                .find((d) => {
                                    if (Array.isArray(d)) {
                                        return d.find((v) => signers.includes(v.toHex()));
                                    } else {
                                        return signers.includes(d.toHex());
                                    }
                                })!
                                .toHex()
                        ];
                    return signerIndexA - signerIndexB;
                });

            //There is no good compatibility method here.Only successful and failed events can be filtered normally, but it cannot filter error + successful events, which may need further optimization
            const eventsToUse = filtered_events_with_signer.length > 0 ? filtered_events_with_signer : filtered_events;

            events = [...eventsToUse];

            if (events.length === txsLength) {
                resolve(events.map((e) => e.event));

                unsubscribe();

                return;
            }
        });
    });
}

export async function sendTxsWithUtility(
    context: IntegrationTestContext,
    signer: KeyringPair,
    txs: TransactionSubmit[],
    pallet: string,
    events: string[]
): Promise<string[] | Event[]> {
    //ensure the tx is in block
    const isInBlockPromise = new Promise((resolve) => {
        context.api.tx.utility.batchAll(txs.map(({ tx }) => tx)).signAndSend(signer, async (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve(result.status);
            } else if (result.status.isInvalid) {
                console.log(`Transaction is ${result.status}`);
            }
        });
    });

    await isInBlockPromise;

    const resp_events = (await listenEvent(context.api, pallet, events, txs.length, [
        u8aToHex(signer.addressRaw),
    ])) as any;

    expect(resp_events.length).to.be.equal(txs.length);
    return resp_events;
}

export async function multiAccountTxSender(
    context: IntegrationTestContext,
    txs: TransactionSubmit[],
    signers: KeyringPair | KeyringPair[],
    pallet: string,
    events: string[]
): Promise<Event[]> {
    let signers_hex: HexString[] = [];
    if (Array.isArray(signers)) {
        for (let index = 0; index < signers.length; index++) {
            signers_hex.push(u8aToHex(signers[index].addressRaw));
        }
    } else {
        signers_hex.push(u8aToHex(signers.addressRaw));
    }

    await sendTxUntilInBlockList(context.api, txs, signers);
    const resp_events = await listenEvent(context.api, pallet, events, txs.length, signers_hex);
    expect(resp_events.length).to.be.equal(txs.length);
    return resp_events;
}
