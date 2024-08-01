import { expect } from 'chai'
import { ethers } from 'hardhat'
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers'
import {
    assembleSchemaUrl,
    deployContract,
    deployMockContract,
    expectAssertionResult,
    IdentityType,
    Web3Network,
    Op,
} from './utils/helper'

describe('TokenHoldingAmount', () => {
    const deployFixture = async () => {
        await deployMockContract('MockHttpGetString')
        await deployMockContract('MockHttpGetI64')
        await deployMockContract('MockIdentityToString')
        await deployMockContract('MockParseDecimal')
        return await deployContract('TokenMapping')
    }

    const expectResult = async (
        contract: any,
        val: any,
        assertion: any,
        result: boolean
    ) => {
        await expectAssertionResult(contract, val, {
            tokenType: 'Token Holding Amount',
            tokenDesc: 'The amount of a particular token you are holding',
            schemaUrl: assembleSchemaUrl('25-token-holding-amount/1-1-3.json'),
            assertions: [assertion],
            result,
        })
    }

    const generateParams = (token: string) =>
        ethers.AbiCoder.defaultAbiCoder().encode(['string'], [token])

    describe('BRC20', () => {
        const secrets = ['0x12345', '0x12345']

        const expectOrdiFalseResult = (contract: any, val: any) =>
            expectResult(
                contract,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'ordi',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '0',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '1',
                        },
                    ],
                },
                false
            )

        it('should return result false when amount = 0', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '13MG12FZ2bDzWrLy2pQGvpq6LdYjw2sfmy'
                        ),
                        networks: [Web3Network.BitcoinP2sh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectOrdiFalseResult(TokenMapping, val)
        })

        it('should return result true when amount < 1', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '1DtMsV2JDUVccDU5V5zdV4mdBf5KRHaJ7Z'
                        ),
                        networks: [Web3Network.BitcoinP2tr],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'ordi',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '0',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '1',
                        },
                    ],
                },
                true
            )
        })

        it('should return result true when amount = 1', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '17dUgEh3jSnGrNmtMaPrVHvVgjNiqkHHhb'
                        ),
                        networks: [Web3Network.BitcoinP2tr],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'ordi',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '1',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '5',
                        },
                    ],
                },
                true
            )
        })

        it('should return result true when amount > 1', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '1BCtecRbWLi1NYzfj9CNszJhCh3c2LXGPd'
                        ),
                        networks: [Web3Network.BitcoinP2wsh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'ordi',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '1',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '5',
                        },
                    ],
                },
                true
            )
        })

        it('should return result true when amount > 500', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '1MWAjD8eSqHro35WVcWV3N3VGfyzCsiMVM'
                        ),
                        networks: [Web3Network.BitcoinP2pkh],
                    },
                ],
                // secrets
                ['0x12345', '0x12345'],
                // params
                generateParams('ordi')
            )
            await expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'ordi',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '500',
                        },
                    ],
                },
                true
            )
        })

        it('should return result false if fail to call identity_to_string', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes('identity_to_string_fail'),
                        networks: [Web3Network.BitcoinP2sh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectOrdiFalseResult(TokenMapping, val)
        })

        it('should return result false if fail to call http_get_string', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes('httt_get_string_fail'),
                        networks: [Web3Network.BitcoinP2sh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectOrdiFalseResult(TokenMapping, val)
        })

        it('should return result false if fail to call parse_decimal_fail', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes('parse_decimal_fail'),
                        networks: [Web3Network.BitcoinP2sh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('ordi')
            )
            await expectOrdiFalseResult(TokenMapping, val)
        })

        it('should throw eror if token name not exist', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            '17fQAnS9FSUw5zBYY26DURYE4PXiemNKgb'
                        ),
                        networks: [Web3Network.BitcoinP2sh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('not_exist_token_name')
            )
            await expect(val).to.be.rejectedWith(
                Error,
                `VM Exception while processing transaction: reverted with reason string 'Token not supported or not found'`
            )
        })
    })

    describe('Btc', () => {
        const secrets = ['0x12345', '0x12345']
        it('should return result false when amount = 0', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            'bc1pqdk57wus42wuh989k3v700n6w584andwg7pvxnrd69ag3rs94cfq40qx2y'
                        ),
                        networks: [Web3Network.BitcoinP2wpkh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('btc')
            )
            expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'btc',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '0',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '0.001',
                        },
                    ],
                },
                false
            )
        })
        it('should return result true when amount < 0.3', async () => {
            const { TokenMapping } = await loadFixture(deployFixture)
            const val = TokenMapping.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Bitcoin,
                        value: ethers.toUtf8Bytes(
                            'bc1pg6qjsrxwg9cvqx0gxstl0t74ynhs2528t7rp0u7acl6etwn5t6vswxrzpa'
                        ),
                        networks: [Web3Network.BitcoinP2wpkh],
                    },
                ],
                // secrets
                secrets,
                // params
                generateParams('btc')
            )
            await expectResult(
                TokenMapping,
                val,
                {
                    and: [
                        {
                            src: '$token',
                            op: Op.EQ,
                            dst: 'btc',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.GTE,
                            dst: '0.1',
                        },
                        {
                            src: '$holding_amount',
                            op: Op.LT,
                            dst: '0.3',
                        },
                    ],
                },
                true
            )
        })
    })
})
