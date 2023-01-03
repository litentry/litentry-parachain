import 'mocha';

import '@polkadot/api-augment';
import { Contract, ethers, Wallet } from 'ethers';
import { BN } from '@polkadot/util';
import fs from 'fs';
import { spawn } from 'child_process';
import { initApiPromise, loadConfig, ParachainConfig, signAndSend, sleep, sudoWrapper } from './utils';
import { toWei } from 'web3-utils';

const path = require('path');
const BridgeContract = require('../bridge/contracts/Bridge.json');
const ERC20HandlerContract = require('../bridge/contracts/ERC20Handler.json');
const ERC721HandlerContract = require('../bridge/contracts/ERC721Handler.json');
const GenericHandlerContract = require('../bridge/contracts/GenericHandler.json');
const ERC20Contract = require('../bridge/contracts/MintableERC20.json');

class EthConfig {
    wallets!: { alice: Wallet; bob: Wallet; charlie: Wallet; dave: Wallet; eve: Wallet };
    bridge!: Contract;
    erc20Handler!: Contract;
    erc721Handler!: Contract;
    genericHandler!: Contract;
    erc20!: Contract;
}

function generateTestKeys(): { alice: string; bob: string; charlie: string; dave: string; eve: string } {
    const secp256k1PrivateKeyLength = 32;
    const names = ['alice', 'bob', 'charlie', 'dave', 'eve'];
    let keys = new Array<string>();
    for (const name of names) {
        const result = Buffer.alloc(secp256k1PrivateKeyLength);
        result.fill(name, secp256k1PrivateKeyLength - Buffer.from(name, 'utf8').length);
        keys.push(result.toString('hex'));
    }
    return { alice: keys[0], bob: keys[1], charlie: keys[2], dave: keys[3], eve: keys[4] };
}

async function deployBridgeContracts(wallet: Wallet) {
    const evmChainID = 0;
    const initialRelayers = new Array<String>();
    const threshold = 1;
    const fee = 0;
    const expiry = 100;
    const initialResourceIDs = new Array<String>();
    const initialContractAddresses = new Array<String>();
    const burnableContractAddresses = new Array<String>();
    const initialDepositFunctionSignatures = new Array<String>();
    const initialExecuteFunctionSignatures = new Array<String>();
    const tokenName = 'Litentry';
    const symbol = 'LIT';
    const initialSupply = 0;
    const owner = wallet.address;

    const BridgeFactory = new ethers.ContractFactory(BridgeContract.abi, BridgeContract.bytecode, wallet);
    const ERC20HandlerFactory = new ethers.ContractFactory(
        ERC20HandlerContract.abi,
        ERC20HandlerContract.bytecode,
        wallet
    );
    const ERC721HandlerFactory = new ethers.ContractFactory(
        ERC721HandlerContract.abi,
        ERC721HandlerContract.bytecode,
        wallet
    );
    const GenericHandlerFactory = new ethers.ContractFactory(
        GenericHandlerContract.abi,
        GenericHandlerContract.bytecode,
        wallet
    );
    const ERC20Factory = new ethers.ContractFactory(ERC20Contract.abi, ERC20Contract.bytecode, wallet);

    // deploy contracts
    const bridge = await BridgeFactory.deploy(evmChainID, initialRelayers, threshold, fee, expiry);
    const erc20Handler = await ERC20HandlerFactory.deploy(
        bridge.address,
        initialResourceIDs,
        initialContractAddresses,
        burnableContractAddresses
    );
    const erc721Handler = await ERC721HandlerFactory.deploy(
        bridge.address,
        initialResourceIDs,
        initialContractAddresses,
        burnableContractAddresses
    );
    const genericHandler = await GenericHandlerFactory.deploy(
        bridge.address,
        initialResourceIDs,
        initialContractAddresses,
        initialDepositFunctionSignatures,
        initialExecuteFunctionSignatures
    );
    const erc20 = await ERC20Factory.deploy(tokenName, symbol, initialSupply, owner);

    console.log('Bridge:         ', bridge.address);
    console.log('ERC20Handler:   ', erc20Handler.address);
    console.log('ERC721Handler:  ', erc721Handler.address);
    console.log('GenericHandler: ', genericHandler.address);
    console.log('ERC20:          ', erc20.address);

    await sleep(1);
    return { bridge, erc20Handler, erc721Handler, genericHandler, erc20 };
}

async function setupCrossChainTransfer(
    pConfig: ParachainConfig,
    eConfig: EthConfig,
    ethRelayers: [string],
    parachainRelayers: [string]
) {
    let opts = { gasLimit: 85000, gasPrice: 20000000000 };
    const parachainFee = new BN(10).pow(new BN(12)); // 1 unit
    const sourceChainID = 0; //ethereum
    const destChainID = parseInt(pConfig.api.consts.chainBridge.bridgeChainId.toString()); //parachain
    const depositNonce = await pConfig.api.query.chainBridge.votes.entries(sourceChainID);

    const destResourceId = pConfig.api.consts.bridgeTransfer.nativeTokenResourceId.toHex();
    await eConfig.erc20.mint(eConfig.wallets.alice.address, toWei('100000'));
    await eConfig.erc20.mint(eConfig.wallets.bob.address, toWei('100000'));
    await eConfig.erc20.mint(eConfig.wallets.charlie.address, toWei('100000'));
    await eConfig.erc20.mint(eConfig.wallets.dave.address, toWei('100000'));
    await eConfig.erc20.mint(eConfig.wallets.eve.address, toWei('100000'));
    await eConfig.erc20.mint(eConfig.erc20Handler.address, toWei('300'));
    await eConfig.bridge.adminSetResource(eConfig.erc20Handler.address, destResourceId, eConfig.erc20.address);
    await eConfig.bridge.adminSetDecimals(eConfig.erc20Handler.address, eConfig.erc20.address, 18, 12, opts);
    //  votes.entries equivalent to nonce
    await eConfig.bridge.adminSetDepositNonce(destChainID, depositNonce.length, opts);
    for (let i = 0; i < ethRelayers.length; i++) {
        await eConfig.bridge.adminAddRelayer(ethRelayers[i]);
    }
    const MINTER_ROLE = await eConfig.erc20.MINTER_ROLE();
    await eConfig.erc20.grantRole(MINTER_ROLE, eConfig.erc20Handler.address);

    // parachain setup
    let extrinsic = [];
    for (let i = 0; i < parachainRelayers.length; i++) {
        const isRelayer = await pConfig.api.query.chainBridge.relayers(parachainRelayers[i]);
        if (!isRelayer.toHuman()) {
            extrinsic.push(await sudoWrapper(pConfig.api, pConfig.api.tx.chainBridge.addRelayer(parachainRelayers[i])));
        }
    }

    // const filterMode = (await pConfig.api.query.extrinsicFilter.mode()).toHuman();
    // if ('Test' !== filterMode) {
    //     extrinsic.push(pConfig.api.tx.sudo.sudo(pConfig.api.tx.extrinsicFilter.setMode('Test')));
    // }

    const whitelist = await pConfig.api.query.chainBridge.chainNonces(sourceChainID);
    if (!whitelist.toHuman()) {
        extrinsic.push(await sudoWrapper(pConfig.api, pConfig.api.tx.chainBridge.whitelistChain(sourceChainID)));
    }

    const resource = await pConfig.api.query.chainBridge.resources(destResourceId);
    if (resource.toHuman() !== 'BridgeTransfer.transfer') {
        extrinsic.push(
            await sudoWrapper(
                pConfig.api,
                pConfig.api.tx.chainBridge.setResource(destResourceId, 'BridgeTransfer.transfer')
            )
        );
    }

    const fee = await pConfig.api.query.chainBridge.bridgeFee(sourceChainID);
    if (!fee || fee.toString() !== parachainFee.toString()) {
        extrinsic.push(await sudoWrapper(pConfig.api, pConfig.api.tx.chainBridge.updateFee(0, parachainFee)));
    }

    if (extrinsic.length > 0) {
        const tx = pConfig.api.tx.utility.batch(extrinsic);
        await signAndSend(tx, pConfig.alice);
    }
}

function generateBridgeConfig(
    eConfig: EthConfig,
    ethRelayer: string,
    parachainRelayer: string,
    ethStartFrom: number,
    parachainStartFrom: number,
    parachainChainID: number,
    filename: string
) {
    // import sub key: chainbridge accounts import --sr25519 --privateKey //Alice
    // import eth key: chainbridge accounts import --ethereum ./scripts/geth/keystore/alice.json
    let config = {
        chains: [
            {
                name: 'eth',
                type: 'ethereum',
                id: '0',
                endpoint: 'ws://localhost:8546',
                from: ethRelayer,
                opts: {
                    bridge: eConfig.bridge.address,
                    erc20Handler: eConfig.erc20Handler.address,
                    erc721Handler: eConfig.erc721Handler.address,
                    genericHandler: eConfig.genericHandler.address,
                    gasLimit: '8000000',
                    startBlock: `${ethStartFrom}`,
                    maxGasPrice: '3000000000',
                    blockConfirmations: '2',
                },
            },
            {
                name: 'sub',
                type: 'substrate',
                id: parachainChainID.toString(),
                endpoint: 'ws://127.0.0.1:9944',
                from: parachainRelayer,
                opts: {
                    useExtendedCall: 'true',
                    startBlock: `${parachainStartFrom}`,
                },
            },
        ],
    };
    let data = JSON.stringify(config, null, 4);
    fs.writeFileSync(filename, data);
}

function emptyDir(directoryPath: string) {
    const files = fs.readdirSync(directoryPath);
    for (const file of files) {
        fs.unlinkSync(path.join(directoryPath, file));
    }
}

async function startChainBridge(
    ethConfig: EthConfig,
    parachainConfig: ParachainConfig,
    ethRelayer: string,
    parachainRelayer: string,
    bridgePath: string,
    config: string,
    log: string
) {
    require('dotenv').config();
    const dataDir = './bridge/data';
    if (!fs.existsSync(dataDir)) {
        fs.mkdirSync(dataDir, {recursive: true});
    }
    emptyDir(dataDir);
    const ethBlock = await ethConfig.wallets.bob.provider.getBlockNumber();
    const subBlock = await parachainConfig.api.rpc.chain.getHeader();
    const parachainChainID = parseInt(parachainConfig.api.consts.chainBridge.bridgeChainId.toString()); //parachain

    generateBridgeConfig(
        ethConfig,
        ethRelayer,
        parachainRelayer,
        ethBlock,
        subBlock.number.toNumber(),
        parachainChainID,
        config
    );
    const logging = fs.createWriteStream(log, { flags: 'w+' });
    const lsProcess = spawn(
        // `${process.env.GOPATH}/bin/chainbridge`,
        bridgePath,
        ['--verbosity', 'trace', '--blockstore', dataDir, '--config', config, '--keystore', './bridge/keys'],
        {env: {STAGE: 'dev'}}
    );
    lsProcess.stdout.pipe(logging);
    lsProcess.stderr.pipe(logging);
    lsProcess.on('close', (code) => {
        logging.close();
        console.log(code);
    });
    await sleep(1);
}

export function createERCDepositData(tokenAmountOrID: string, lenRecipientAddress: number, recipientAddress: string) {
    const toHex = (covertThis: string | number, padding: number) => {
        return ethers.utils.hexZeroPad(ethers.utils.hexlify(covertThis), padding);
    };
    return (
        '0x' +
        ethers.utils.hexZeroPad(tokenAmountOrID, 32).substr(2) + // Token amount or ID to deposit (32 bytes)
        ethers.utils.hexZeroPad(ethers.utils.hexlify(lenRecipientAddress), 32).substr(2) + // len(recipientAddress)          (32 bytes)
        recipientAddress.substr(2)
    ); // recipientAddress               (?? bytes)
}

export function describeCrossChainTransfer(
    title: string,
    specFilename: string,
    cb: (context: { ethConfig: EthConfig; parachainConfig: ParachainConfig }) => void
) {
    describe(title, function () {
        this.timeout(6000000);

        let context: { ethConfig: EthConfig; parachainConfig: ParachainConfig } = {
            ethConfig: {} as EthConfig,
            parachainConfig: {} as ParachainConfig,
        };

        before('Deploying Bridge Contracts', async function () {
            const config = loadConfig();
            const parachainConfig = await initApiPromise(config);

            const provider = new ethers.providers.JsonRpcProvider(config.eth_endpoint);
            const wallets = {
                alice: new ethers.Wallet(generateTestKeys().alice, provider),
                bob: new ethers.Wallet(generateTestKeys().bob, provider),
                charlie: new ethers.Wallet(generateTestKeys().charlie, provider),
                dave: new ethers.Wallet(generateTestKeys().dave, provider),
                eve: new ethers.Wallet(generateTestKeys().eve, provider),
            };
            const {
                bridge,
                erc20Handler,
                erc721Handler,
                genericHandler,
                erc20
            } = await deployBridgeContracts(
                wallets.alice
            );

            const ethConfig: EthConfig = {
                bridge,
                erc20,
                erc20Handler,
                erc721Handler,
                genericHandler,
                wallets,
            };

            await setupCrossChainTransfer(
                parachainConfig,
                ethConfig,
                [ethConfig.wallets.bob.address],
                [parachainConfig.bob.address]
            );

            context.ethConfig = ethConfig;
            context.parachainConfig = parachainConfig;

            await startChainBridge(
                ethConfig,
                parachainConfig,
                ethConfig.wallets.bob.address,
                parachainConfig.bob.address,
                config.bridge_path,
                './bridge/bob.json',
                '/tmp/parachain_dev/bob.log'
            );
            await sleep(5);
        });

        after(async function () {
        });

        cb(context);
    });
}
