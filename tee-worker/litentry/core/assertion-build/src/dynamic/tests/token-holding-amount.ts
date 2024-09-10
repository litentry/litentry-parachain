import { expect } from 'chai'
import { ethers } from 'hardhat'
import { loadFixture } from '@nomicfoundation/hardhat-network-helpers'
import {
    assembleSchemaUrl,
    deployContract,
    deployAllMockContracts,
    expectAssertionResult,
    IdentityType,
    Web3Network,
    Op,
} from './utils/helper'

describe('TokenHoldingAmount', () => {
    const deployFixture = async () => {
        await deployAllMockContracts()
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
            schemaUrl: assembleSchemaUrl('25-token-holding-amount/1-1-4.json'),
            assertions: [assertion],
            result,
        })
    }

    const generateParams = (token: string) =>
        ethers.AbiCoder.defaultAbiCoder().encode(['string'], [token])

    const secrets = ['0x12345', '0x12345', '0x12345']

    it('should throw error if token name not exist', async () => {
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

    describe('BRC20', () => {
        const networkClause = {
            and: [
                { src: '$network', op: '==', dst: 'BitcoinP2tr' },
                { src: '$network', op: '==', dst: 'BitcoinP2pkh' },
                { src: '$network', op: '==', dst: 'BitcoinP2sh' },
                { src: '$network', op: '==', dst: 'BitcoinP2wpkh' },
                { src: '$network', op: '==', dst: 'BitcoinP2wsh' },
            ],
        }

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
                        networkClause,
                        {
                            src: '$holding_amount',
                            op: Op.GT,
                            dst: '0',
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
                        networkClause,
                        {
                            src: '$holding_amount',
                            op: Op.GT,
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
                        networkClause,
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
                        networkClause,
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
                        networkClause,
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
    })

    describe('Btc', () => {
        const networkClause = {
            and: [
                { src: '$network', op: '==', dst: 'BitcoinP2tr' },
                { src: '$network', op: '==', dst: 'BitcoinP2pkh' },
                { src: '$network', op: '==', dst: 'BitcoinP2sh' },
                { src: '$network', op: '==', dst: 'BitcoinP2wpkh' },
                { src: '$network', op: '==', dst: 'BitcoinP2wsh' },
            ],
        }

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
                        networkClause,
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
                        networkClause,
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

    describe('ERC20', () => {
        describe('Atom', () => {
            const tokenName = 'atom'

            const networkClause = {
                and: [
                    { src: '$network', op: '==', dst: 'Ethereum' },
                    { src: '$network', op: '==', dst: 'Bsc' },
                    { src: '$network', op: '==', dst: 'Polygon' },
                ],
            }

            it('should return result false when amount = 0', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c'
                            ),
                            networks: [Web3Network.Bsc],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                        ],
                    },
                    false
                )
            })

            it('should return result true when amount > 0 && amount < 1', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xA7Ee59E733E613CC957FE203A2935E85cE39D08A'
                            ),
                            networks: [Web3Network.Ethereum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                            { src: '$holding_amount', op: '<', dst: '1' },
                        ],
                    },
                    true
                )
            })

            it('should return result true when amount = 2', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xA7Ee59E733E613CC957FE203A2935E85cE39D08A'
                            ),
                            networks: [Web3Network.Bsc],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '1' },
                            { src: '$holding_amount', op: '<', dst: '5' },
                        ],
                    },
                    true
                )
            })

            it('should return result true when amount = 30', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c'
                            ),
                            networks: [Web3Network.Polygon],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '20' },
                            { src: '$holding_amount', op: '<', dst: '50' },
                        ],
                    },
                    true
                )
            })

            it('should return result true when amount = 50', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c'
                            ),
                            networks: [Web3Network.Ethereum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '50' },
                            { src: '$holding_amount', op: '<', dst: '80' },
                        ],
                    },
                    true
                )
            })

            it('should return result true when amount == 80', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c'
                            ),
                            networks: [
                                Web3Network.Bsc,
                                Web3Network.Ethereum,
                                Web3Network.Polygon,
                            ],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '80' },
                        ],
                    },
                    true
                )
            })
        })

        describe('Bean', () => {
            const tokenName = 'bean'

            const networkClause = {
                and: [
                    { src: '$network', op: '==', dst: 'Bsc' },
                    { src: '$network', op: '==', dst: 'Combo' },
                ],
            }

            it('should return result false when amount = 0', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xF4d1E80823D7b6BA4A041C58202039611B253590'
                            ),
                            networks: [Web3Network.Bsc, Web3Network.Combo],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                        ],
                    },
                    false
                )
            })

            it('should return result true when amount = 1500', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xa298cA90a4aa6029e26Dacc33b85c3847875615e'
                            ),
                            networks: [Web3Network.Bsc, Web3Network.Combo],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '1500' },
                            { src: '$holding_amount', op: '<', dst: '5000' },
                        ],
                    },
                    true
                )
            })

            it('should return result true when amount = 60000', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x0d4E9A8E1c26747c3d62a883b0Af5a916D6985c5'
                            ),
                            networks: [Web3Network.Bsc, Web3Network.Combo],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>=', dst: '50000' },
                        ],
                    },
                    true
                )
            })
        })

        describe('Dai', () => {
            const tokenName = 'dai'

            const networkClause = {
                and: [
                    { src: '$network', op: '==', dst: 'Ethereum' },
                    { src: '$network', op: '==', dst: 'Bsc' },
                    { src: '$network', op: '==', dst: 'Solana' },
                    { src: '$network', op: '==', dst: 'Arbitrum' },
                    { src: '$network', op: '==', dst: 'Polygon' },
                ],
            }

            it('should return result false when amount = 0', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xF4d1E80823D7b6BA4A041C58202039611B253590'
                            ),
                            networks: [Web3Network.Arbitrum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                        ],
                    },
                    false
                )
            })

            it('should return result true when amount = 5', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xbF98D4df371c2dE965a36E02b4c2E0DA89090818'
                            ),
                            networks: [Web3Network.Arbitrum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                            { src: '$holding_amount', op: '<', dst: '10' },
                        ],
                    },
                    true
                )
            })
        })

        describe('Wbtc', () => {
            const tokenName = 'wbtc'

            const networkClause = {
                and: [{ src: '$network', op: '==', dst: 'Ethereum' }],
            }

            it('should return result false when amount = 0', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0xF4d1E80823D7b6BA4A041C58202039611B253590'
                            ),
                            networks: [Web3Network.Ethereum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                        ],
                    },
                    false
                )
            })

            it('should return result true when amount = 0.0001', async () => {
                const { TokenMapping } = await loadFixture(deployFixture)
                const val = TokenMapping.execute(
                    // identities
                    [
                        {
                            identity_type: IdentityType.Evm,
                            value: ethers.toUtf8Bytes(
                                '0x1C89Edd4FC080D71F92701C0794a16DbE573d4B8'
                            ),
                            networks: [Web3Network.Ethereum],
                        },
                    ],
                    // secrets
                    secrets,
                    // params
                    generateParams(tokenName)
                )
                await expectResult(
                    TokenMapping,
                    val,
                    {
                        and: [
                            { src: '$token', op: '==', dst: tokenName },
                            networkClause,
                            { src: '$holding_amount', op: '>', dst: '0' },
                            { src: '$holding_amount', op: '<', dst: '0.001' },
                        ],
                    },
                    true
                )
            })
        })
    })
})
