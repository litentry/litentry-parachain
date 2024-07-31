import { ethers, run } from 'hardhat'
import WebSocket from 'ws'
import WebSocketAsPromised from 'websocket-as-promised'
import type Options from 'websocket-as-promised/types/options'
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api'
import { compactStripLength, hexToU8a, u8aToString } from '@polkadot/util'
import crypto, { KeyObject } from 'crypto'

// TODO use type from parachain-api instead
// parachain-api using es-module, dynamic using commonjs, cannot import es-module package in commonjs now.
const types = {
    WorkerRpcReturnValue: {
        value: 'Vec<u8>',
    },
}

enum Chain {
    LOCAL = 'local',
    DEV = 'dev',
    STG = 'stg',
    PROD = 'prod',
}

const SUPPORTED_CHAINS: string[] = [
    Chain.LOCAL,
    Chain.DEV,
    Chain.STG,
    Chain.PROD,
]

const CONFIGS: {
    [key: string]: { endpoint: string; trustedEndpoint: string }
} = {
    [Chain.LOCAL]: {
        endpoint: 'ws://127.0.0.1:9944',
        trustedEndpoint: 'ws://127.0.0.1:2000',
    },
    [Chain.DEV]: {
        endpoint: 'wss://tee-dev.litentry.io',
        trustedEndpoint: 'ws://tee-dev.litentry.io:2000',
    },
    [Chain.STG]: {
        endpoint: 'wss://tee-staging.litentry.io',
        trustedEndpoint: 'ws://tee-staging.litentry.io:2000',
    },
    [Chain.PROD]: {
        endpoint: 'wss://tee-prod.litentry.io',
        trustedEndpoint: 'ws://tee-prod.litentry.io:2000',
    },
}

type DeployResult = {
    success: boolean
    // deploy related block hash
    hashes: string[]
}

async function retrieveTeeShieldingKey(api: ApiPromise, endpoint: string) {
    const wsp = new WebSocketAsPromised(endpoint, <Options>(<any>{
        createWebSocket: (url: any) => new WebSocket(url),
        extractMessageData: (event: any) => event,
        packMessage: (data: any) => JSON.stringify(data),
        unpackMessage: (data: string | ArrayBuffer | Blob) =>
            JSON.parse(data.toString()),
        attachRequestId: (data: any, requestId: string | number) =>
            Object.assign({ id: requestId }, data),
        extractRequestId: (data: any) => data && data.id,
    }))
    await wsp.open()

    const responsePromise = new Promise<any>((resolve) =>
        wsp.onMessage.addListener((data) => {
            const parsed = JSON.parse(data)
            wsp.onMessage.removeAllListeners()
            resolve(parsed)
        })
    )

    wsp.sendRequest({
        jsonrpc: '2.0',
        method: 'author_getShieldingKey',
        params: Uint8Array.from([]),
        id: 1,
    })

    const resp = await responsePromise
    const result = resp.result
    const res: any = api.createType('WorkerRpcReturnValue', result)

    const key = JSON.parse(
        u8aToString(compactStripLength(hexToU8a(res.value.toHex()))[1])
    ) as {
        n: Uint8Array
        e: Uint8Array
    }

    return crypto.createPublicKey({
        key: {
            alg: 'RSA-OAEP-256',
            kty: 'RSA',
            use: 'enc',
            n: Buffer.from(key.n.reverse()).toString('base64url'),
            e: Buffer.from(key.e.reverse()).toString('base64url'),
        },
        format: 'jwk',
    })
}

const waitForDeploy = async (api: ApiPromise): Promise<DeployResult> => {
    return new Promise<DeployResult>((resolve) => {
        let blocksToScan = 15
        let deploySuccess = false
        let relatedHashes: string[] = []
        const unsubscribe = api.rpc.chain.subscribeNewHeads(
            async (blockHeader) => {
                process.stdout.write('...')
                const shiftedApi = await api.at(blockHeader.hash)

                const allBlockEvents = await shiftedApi.query.system.events()

                allBlockEvents
                    // @ts-ignore
                    .forEach(
                        // @ts-ignore
                        ({ event, phase }) => {
                            if (phase.isApplyExtrinsic) {
                                if (
                                    event.section === 'evmAssertions' ||
                                    (event.section === 'developerCommittee' &&
                                        event.method === 'MemberExecuted')
                                ) {
                                    const blockHash = blockHeader.hash.toHuman()
                                    if (!relatedHashes.includes(blockHash)) {
                                        relatedHashes.push(blockHash)
                                    }
                                    console.log(
                                        `\n${event.section}.${event.method}, ${blockHash}`
                                    )
                                    if (
                                        event.section === 'evmAssertions' &&
                                        event.method === 'AssertionStored'
                                    ) {
                                        deploySuccess = true
                                    }
                                }
                            }
                        }
                    )

                if (!deploySuccess) {
                    blocksToScan -= 1
                    if (blocksToScan < 1) {
                        console.log(
                            '\nTimed out listening for deploy result event'
                        )
                        resolve({
                            success: deploySuccess,
                            hashes: relatedHashes,
                        })
                        ;(await unsubscribe)()
                    }
                    return
                }

                resolve({ success: deploySuccess, hashes: relatedHashes })
                ;(await unsubscribe)()
            }
        )
    })
}

function generateContractId(contractByteCodes: string, secrets: string[]) {
    const data = contractByteCodes + secrets.join(' ')
    const hash = crypto.createHash('sha256').update(data).digest('hex')
    const contractId = `0x${hash.slice(0, 40)}`
    return contractId
}

function genesisSubstrateWallet(name: string) {
    const keyring = new Keyring({ type: 'sr25519' })
    const keyPair = keyring.addFromUri(`//${name}`, { name })
    return keyPair
}

async function main() {
    const chain = process.env.CHAIN as string
    const contract = process.env.CONTRACT as string
    const secrets = (process.env.SECRETS as string)
        .split(' ')
        .filter((secret) => !!secret)

    if (!SUPPORTED_CHAINS.includes(chain)) {
        throw new Error(
            `Unsupported chain ${chain}, need be one of ${SUPPORTED_CHAINS}.`
        )
    }

    const { endpoint, trustedEndpoint } = CONFIGS[chain]

    const api = await ApiPromise.create({
        provider: new WsProvider(endpoint),
        types,
    })

    const encryptedSecrets: string[] = []
    if (secrets.length > 0) {
        let teeShieldingKey: KeyObject
        try {
            teeShieldingKey = await retrieveTeeShieldingKey(
                api,
                trustedEndpoint
            )
        } catch (e) {
            throw new Error(`Fail to retrieve teeShieldingKey, error: ${e}`)
        }

        secrets.forEach((secret) => {
            const secretEncoded = api.createType('String', secret).toU8a()
            const encryptedSecret = crypto.publicEncrypt(
                {
                    key: teeShieldingKey,
                    padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
                    oaepHash: 'sha256',
                },
                secretEncoded
            )
            encryptedSecrets.push(`0x${encryptedSecret.toString('hex')}`)
        })
    }

    const ContractFactory = await ethers.getContractFactory(contract)
    const contractId = generateContractId(ContractFactory.bytecode, secrets)

    console.log(
        `Begin to deploying contract: ${contract}, to chain: ${chain}, contract id: ${contractId}`
    )
    const waitForDeployPromise = waitForDeploy(api)

    const proposal = api.tx.evmAssertions.createAssertion(
        contractId,
        ContractFactory.bytecode,
        encryptedSecrets
    )
    const alice = genesisSubstrateWallet('Alice')
    await api.tx.developerCommittee
        .execute(proposal, proposal.encodedLength)
        .signAndSend(alice)

    const result = await waitForDeployPromise
    if (result.success) {
        console.log(
            `Success deploy contract: ${contract}, to chain: ${chain}, contract id: ${contractId}`
        )
    } else {
        console.log(
            'Deploy failed, check the failure reason in these block detail below:'
        )
        result.hashes.forEach((hash) => {
            console.log(
                `https://polkadot.js.org/apps/?rpc=${endpoint}#/explorer/query/${hash}`
            )
        })
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error)
        process.exit(1)
    })
