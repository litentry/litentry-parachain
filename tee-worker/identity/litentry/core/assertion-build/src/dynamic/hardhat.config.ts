import { HardhatUserConfig } from 'hardhat/config'
import '@nomicfoundation/hardhat-toolbox'
const config: HardhatUserConfig = {
    solidity: '0.8.11',
    paths: {
        tests: './tests',
    },
    networks: {
        hardhat: {
            allowUnlimitedContractSize: true,
            accounts: {
                accountsBalance: '1000000000000000000000',
            },
            blockGasLimit: 1000000000,
        },
    },
}

export default config
