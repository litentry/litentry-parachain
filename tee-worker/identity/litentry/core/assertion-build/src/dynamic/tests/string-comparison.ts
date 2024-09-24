import { expect } from 'chai'
import { ethers } from 'hardhat'

describe('StringComparison', () => {
    let stringComparison: any

    before(async () => {
        const StringComparison = await ethers.getContractFactory(
            'StringComparisonTest'
        )
        stringComparison = await StringComparison.deploy()
    })

    it('should correctly compare strings ignoring case', async () => {
        const stringTuples: Array<[string, string, boolean]> = [
            ['Hello World', 'Hello World', true],
            ['Hello World', 'Hello world', true],
            ['hello World', 'Hello World', true],
            ['hello World', 'hello world', true],
            ['hello world', 'Hello World', true],
            ['Hello World', 'Hello World', true],
            ['HELLO WORLD', 'Hello World', true],
            ['HELLO WORLD', 'Hello WORLD', true],
            ['Hello World', 'Hello Worlds', false],
        ]
        for (const stringTuple of stringTuples) {
            const result = await stringComparison.compareStringsIgnoreCase(
                stringTuple[0],
                stringTuple[1]
            )
            expect(result).to.equal(stringTuple[2])
        }
    })
})
