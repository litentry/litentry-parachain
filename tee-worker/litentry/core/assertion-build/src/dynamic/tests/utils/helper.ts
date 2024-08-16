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
    HTTP_GET = '0x00000000000000000000000000000000000003EE',
    HTTP_GET_STRING = '0x00000000000000000000000000000000000003EA',
    HTTP_POST_STRING = '0x00000000000000000000000000000000000003ED',
    HTTP_GET_I64 = '0x00000000000000000000000000000000000003E8',
    HTTP_GET_BOOL = '0x00000000000000000000000000000000000003E9',
    IDENTITY_TO_STRING = '0x000000000000000000000000000000000000041C',
    HEX_TO_NUMBER = '0x000000000000000000000000000000000000041D',
    PARSE_DECIMAL = '0x000000000000000000000000000000000000041E',
    JSON_GET_ARRAY_LEN = '0x000000000000000000000000000000000000044F',
    JSON_GET_STRING = '0x000000000000000000000000000000000000044C',
}

const mockContractAddressMapping: { [key: string]: string } = {
    MockHttpGet: PrecompileAddresses.HTTP_GET,
    MockHttpGetString: PrecompileAddresses.HTTP_GET_STRING,
    MockHttpPostString: PrecompileAddresses.HTTP_POST_STRING,
    MockHttpGetI64: PrecompileAddresses.HTTP_GET_I64,
    MockHttpGetBool: PrecompileAddresses.HTTP_GET_BOOL,
    MockIdentityToString: PrecompileAddresses.IDENTITY_TO_STRING,
    MockHexToNumber: PrecompileAddresses.HEX_TO_NUMBER,
    MockParseDecimal: PrecompileAddresses.PARSE_DECIMAL,
    MockJsonGetArrayLen: PrecompileAddresses.JSON_GET_ARRAY_LEN,
    MockJsonGetString: PrecompileAddresses.JSON_GET_STRING,
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

export async function deployAllMockContracts(): Promise<void> {
    await deployMockContract('MockHttpGet')
    await deployMockContract('MockHttpGetString')
    await deployMockContract('MockHttpPostString')
    await deployMockContract('MockHttpGetI64')
    await deployMockContract('MockIdentityToString')
    await deployMockContract('MockHexToNumber')
    await deployMockContract('MockParseDecimal')
    await deployMockContract('MockJsonGetArrayLen')
    await deployMockContract('MockJsonGetString')
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
