import { ethers } from 'hardhat'
import WebSocket from 'ws'
import { all, getChain } from '@litentry/chaindata'
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

const chainIdPrefix = 'litentry-'

const SUPPORTED_CHAINS: string[] = all.map((item) =>
    item.id.replace(chainIdPrefix, '')
)

const CHAINS_WITHOUT_MNEMONIC = ['local', 'dev']

type DeployResult = {
    success: boolean
    // deploy related block hash
    hashes: string[]
}

function createWebSocket(endpoint: string): Promise<WebSocket> {
    return new Promise((resolve, reject) => {
        const ws = new WebSocket(endpoint)

        ws.on('open', () => {
            console.log('WebSocket connection opened.')
            resolve(ws)
        })

        ws.on('error', (error) => {
            reject(error)
        })

        ws.on('close', (code, reason) => {
            console.log(
                `WebSocket connection closed: ${code}${(reason ?? '').length > 0 ? ` ${reason}` : ''}.`
            )
        })
    })
}

function sendRequest(ws: WebSocket, request: any): Promise<any> {
    return new Promise((resolve, reject) => {
        const messageListener = (data: any) => {
            try {
                const parsed = JSON.parse(data)
                if (parsed.id === request.id) {
                    ws.off('message', messageListener)
                    resolve(parsed)
                }
            } catch (error) {
                reject(error)
            }
        }

        ws.on('message', messageListener)

        ws.send(JSON.stringify(request), (err) => {
            if (err) {
                reject(err)
            }
        })
    })
}

async function retrieveTeeShieldingKey(api: ApiPromise, endpoint: string) {
    const ws = await createWebSocket(endpoint)

    const resp = await sendRequest(ws, {
        jsonrpc: '2.0',
        method: 'author_getShieldingKey',
        params: Uint8Array.from([]),
        id: new Date().getTime(),
    })

    const result = resp.result
    const res: any = api.createType('WorkerRpcReturnValue', result)

    const key = JSON.parse(
        u8aToString(compactStripLength(hexToU8a(res.value.toHex()))[1])
    ) as {
        n: Uint8Array
        e: Uint8Array
    }

    ws.close()

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

/**
 * Generates a contract ID based on the provided contract byte code and secrets.
 *
 * Same contract byte code and secrets will get the same contractId, this can ensure dev/stg/prod has same contract ID.
 *
 * @param {string} contractByteCode - The byte codes of the contract.
 * @param {string[]} secrets - An array of secret strings to be included in the hash.
 * @returns {string} The generated contract ID.
 */
function generateContractId(
    contractByteCode: string,
    secrets: string[]
): string {
    // Combine the contract byte codes and secrets into a single string
    const data = contractByteCode + secrets.join(' ')

    // Create a SHA-256 hash of the combined data
    const hash = crypto.createHash('sha256').update(data).digest('hex')

    // Take the first 40 characters of the hash to form the contract ID, prefixed with '0x'
    const contractId = `0x${hash.slice(0, 40)}`
    return contractId
}

function genesisSubstrateWallet(name: string) {
    const keyring = new Keyring({ type: 'sr25519' })
    const keyPair = keyring.addFromUri(`//${name}`, { name })
    return keyPair
}

function getSenderWallet(mnemonic: string) {
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 42 })
    return keyring.addFromUri(mnemonic, { name: 'ss-sender' }, 'sr25519')
}

/**
 * Encrypts a plaintext buffer using the provided public key in segments.
 *
 * Same logic as: https://github.com/apache/incubator-teaclave-sgx-sdk/blob/master/sgx_crypto_helper/src/rsa3072.rs#L161-L179
 *
 * @param {crypto.KeyLike} pubKey - The public key to use for encryption.
 * @param {Uint8Array} plaintext - The plaintext buffer to encrypt.
 * @returns {Buffer} The encrypted data.
 */
function encryptBuffer(pubKey: crypto.KeyLike, plaintext: Uint8Array): Buffer {
    const bs = 384 // 3072 bits = 384 bytes
    const bsPlain = bs - (2 * 256) / 8 - 2 // Maximum plaintext block size
    const count = Math.ceil(plaintext.length / bsPlain) // Use Math.ceil to ensure proper chunk count

    const cipherText = Buffer.alloc(bs * count)

    for (let i = 0; i < count; i++) {
        const plainSlice = plaintext.slice(
            i * bsPlain,
            Math.min((i + 1) * bsPlain, plaintext.length)
        )
        const cipherSlice = crypto.publicEncrypt(
            {
                key: pubKey,
                padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
                oaepHash: 'sha256',
            },
            plainSlice
        )

        cipherSlice.copy(cipherText, i * bs)
    }

    return cipherText
}

async function main() {
    const chain = process.env.CHAIN as string
    const contract = process.env.CONTRACT as string
    const mnemonic = process.env.MNEMONIC as string
    const secrets = ((process.env.SECRETS as string) ?? '')
        .split('\n')
        .filter((secret) => !!secret)

    if (!SUPPORTED_CHAINS.includes(chain)) {
        throw new Error(
            `Unsupported chain ${chain}, need be one of ${SUPPORTED_CHAINS}.`
        )
    }

    // mnemonic is required for staging and prod chain.
    const needUseMnemonic = !CHAINS_WITHOUT_MNEMONIC.includes(chain)
    if (needUseMnemonic && (mnemonic ?? '').length === 0) {
        throw new Error(`The mnemonic is required for ${chain} chain.`)
    }

    const chainId = `${chainIdPrefix}${chain}`
    const { rpcs, enclaveRpcs } = getChain(chainId)

    const api = await ApiPromise.create({
        provider: new WsProvider(rpcs[0].url),
        types,
    })

    const encryptedSecrets: string[] = []
    if (secrets.length > 0) {
        let teeShieldingKey: KeyObject
        try {
            teeShieldingKey = await retrieveTeeShieldingKey(
                api,
                enclaveRpcs[0].url
            )
        } catch (e) {
            throw new Error(`Fail to retrieve teeShieldingKey, error: ${e}`)
        }

        secrets.forEach((secret) => {
            const encodedSecret = api.createType('String', secret).toU8a()
            // Some secrets are too large, so need using segment encryption.
            const encryptedSecret = encryptBuffer(
                teeShieldingKey,
                encodedSecret
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
    const alice = needUseMnemonic
        ? getSenderWallet(mnemonic)
        : genesisSubstrateWallet('Alice')
    await api.tx.developerCommittee
        .execute(proposal, proposal.encodedLength)
        .signAndSend(alice)

    const result = await waitForDeployPromise
    if (result.success) {
        console.log(
            `Success deploy contract: ${contract}, to chain: ${chain}, contract id: ${contractId}`
        )
        console.log(`Check deployment result in these block details below:`)
    } else if (result.hashes.length > 0) {
        console.log(
            'Deploy failed, check the failure reason in these block details below:'
        )
    }
    result.hashes.forEach((hash) => {
        console.log(
            `https://polkadot.js.org/apps/?rpc=${rpcs[0].url}#/explorer/query/${hash}`
        )
    })
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error)
        process.exit(1)
    })
