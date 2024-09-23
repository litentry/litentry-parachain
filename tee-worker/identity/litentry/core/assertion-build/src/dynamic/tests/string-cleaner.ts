import { expect } from 'chai'
import { ethers } from 'hardhat'

describe('StringCleaner', () => {
    let stringCleaner: any

    before(async () => {
        const StringCleaner =
            await ethers.getContractFactory('StringCleanerTest')
        stringCleaner = await StringCleaner.deploy()
    })

    it('should correctly identify visible and invisible characters', async () => {
        const stringParis: Array<[string, string]> = [
            ['Hello\u200BWorld', 'HelloWorld'],
            ['Line1\nLine2', 'Line1Line2'],
            ['Tab\tSeparated\tValues', 'TabSeparatedValues'],
            ['Carriage\rReturn', 'CarriageReturn'],
            ['Vertical\vTab', 'VerticalTab'],
            ['Form\fFeed', 'FormFeed'],
            ['Spaces    \u00A0\u2007\u202F', 'Spaces    '],
            ['Unicode\uFEFFByteOrderMark', 'UnicodeByteOrderMark'],
        ]
        for (const stringPair of stringParis) {
            const result = await stringCleaner.cleanString(stringPair[0])
            expect(result).to.equal(stringPair[1])
        }
    })
})
