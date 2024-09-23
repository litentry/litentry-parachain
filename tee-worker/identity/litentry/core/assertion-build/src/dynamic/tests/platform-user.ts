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

describe('PlatformUser', () => {
    const deployFixture = async () => {
        await deployMockContract('MockHttpGetBool')
        await deployMockContract('MockIdentityToString')
        return await deployContract('PlatformUser')
    }

    const expectResult = async (
        contract: any,
        val: any,
        platform: string,
        result: boolean
    ) => {
        await expectAssertionResult(contract, val, {
            tokenType: 'Platform user',
            tokenDesc: 'You are a user of a certain platform',
            schemaUrl: assembleSchemaUrl('24-platform-user/1-1-2.json'),
            assertions: [
                { and: [{ src: '$platform', op: Op.EQ, dst: platform }] },
            ],
            result,
        })
    }

    const generateParams = (token: string) =>
        ethers.AbiCoder.defaultAbiCoder().encode(['string'], [token])

    describe('DarenMarket', () => {
        const darenMarketParams = generateParams('DarenMarket')

        const expectDarenMarketResult = (
            contract: any,
            val: any,
            result: boolean
        ) => expectResult(contract, val, 'DarenMarket', result)

        it('should throw error if platform is not support', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes(
                            '0x96aEb2216810C624131c51141da612808103d319'
                        ),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                ],
                [],
                // params
                generateParams('NotSupportedPlatform')
            )
            await expect(val).to.be.rejectedWith(
                Error,
                `VM Exception while processing transaction: reverted with reason string 'Platform not supported'`
            )
        })

        it('should return result false when identity is not support', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
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
                [],
                // params
                darenMarketParams
            )
            await expectDarenMarketResult(PlatformUser, val, false)
        })

        it('should return result false if call api failure', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes('success_false'),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                ],
                [],
                // params
                darenMarketParams
            )
            await expectDarenMarketResult(PlatformUser, val, false)
        })

        it('should return result false if is not platform user', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes(
                            '0x733fB0d0899C1D1952Eb68eb38Fd6e6409fA280e'
                        ),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                ],
                [],
                // params
                darenMarketParams
            )
            await expectDarenMarketResult(PlatformUser, val, false)
        })

        it('should return result true if is platform user', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes(
                            '0x96aEb2216810C624131c51141da612808103d319'
                        ),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                ],
                [],
                // params
                darenMarketParams
            )
            await expectDarenMarketResult(PlatformUser, val, true)
        })

        it('should return result true if any evm identity is platform user', async () => {
            const { PlatformUser } = await loadFixture(deployFixture)
            const val = PlatformUser.execute(
                // identities
                [
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes(
                            '0x733fB0d0899C1D1952Eb68eb38Fd6e6409fA280e'
                        ),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                    {
                        identity_type: IdentityType.Evm,
                        value: ethers.toUtf8Bytes(
                            '0x96aEb2216810C624131c51141da612808103d319'
                        ),
                        networks: [Web3Network.Ethereum, Web3Network.Bsc],
                    },
                ],
                [],
                // params
                darenMarketParams
            )
            await expectDarenMarketResult(PlatformUser, val, true)
        })
    })
})
