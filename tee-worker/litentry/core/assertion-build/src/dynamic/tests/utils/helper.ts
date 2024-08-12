import { ethers } from 'hardhat'
import { expect } from 'chai'

export const ASSERTION_GENERATED_EVENT_NAME = 'DynamicAssertionGenerated'

export enum IdentityType {
    // web2
    Twitter = 0,
    Discord = 1,
    Github = 2,

    // web3
    Substrate = 3,
    Evm = 4,
    Bitcoin = 5,
    Solana = 6,
}

export enum Web3Network {
    // substrate
    Polkadot = 0,
    Kusama = 1,
    Litentry = 2,
    Litmus = 3,
    LitentryRococo = 4,
    Khala = 5,
    SubstrateTestnet = 6,

    // evm
    Ethereum = 7,
    Bsc = 8,
    Polygon = 14,
    Arbitrum = 15,
    Solana = 16,
    Combo = 17,

    // btc
    BitcoinP2tr = 9,
    BitcoinP2pkh = 10,
    BitcoinP2sh = 11,
    BitcoinP2wpkh = 12,
    BitcoinP2wsh = 13,
}

export enum Op {
    GT = '>',
    GTE = '>=',
    EQ = '==',
    LT = '<',
    LTE = '<=',
    NE = '!=',
}

export enum PrecompileAddresses {
    HTTP_GET_STRING = '0x00000000000000000000000000000000000003EA',
    IDENTITY_TO_STRING = '0x000000000000000000000000000000000000041C',
    PARSE_DECIMAL = '0x000000000000000000000000000000000000041E',
    HTTP_GET_I64 = '0x00000000000000000000000000000000000003E8',
}

const mockContractAddressMapping: { [key: string]: string } = {
    MockHttpGetString: PrecompileAddresses.HTTP_GET_STRING,
    MockHttpGetI64: PrecompileAddresses.HTTP_GET_I64,
    MockIdentityToString: PrecompileAddresses.IDENTITY_TO_STRING,
    MockParseDecimal: PrecompileAddresses.PARSE_DECIMAL,
}

export async function deployMockContract(
    contractName: string,
    contractAddress?: string
): Promise<void> {
    const Contract = await ethers.getContractFactory(contractName)
    const contract = await Contract.deploy()

    const _contractAddress =
        mockContractAddressMapping[contractName] ?? contractAddress ?? ''
    await ethers.provider.send('hardhat_setCode', [
        _contractAddress,
        await ethers.provider.getCode(contract.target),
    ])
}

export async function deployContract(contract: string) {
    const SourceContract = await ethers.getContractFactory(contract)
    const sourceContract = await SourceContract.deploy()

    const ProxyContract = await ethers.getContractFactory(
        'ProxyDynamicAssertion'
    )
    const proxyContract = await ProxyContract.deploy(sourceContract.target)

    return { [contract]: proxyContract }
}

export function assembleSchemaUrl(path: string) {
    return `https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/${path}`
}

type ExpectResult = {
    tokenType: string
    tokenDesc: string
    schemaUrl: string
    assertions: any[]
    result: boolean
}

export const expectAssertionResult = async (
    contract: any,
    val: any,
    expectResult: ExpectResult
) => {
    await expect(val)
        .to.emit(contract, ASSERTION_GENERATED_EVENT_NAME)
        .withArgs(
            expectResult.tokenDesc,
            expectResult.tokenType,
            expectResult.assertions.map((a: any) => JSON.stringify(a)),
            expectResult.schemaUrl,
            expectResult.result
        )
}
