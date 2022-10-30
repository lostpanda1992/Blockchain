/// BICONOMY UTILS ///

require('dotenv').config()
const { Biconomy } = require('@biconomy/mexa')
const { ethers } = require('hardhat')
const fetch = require('node-fetch')

const Forwarders = { // biconomy trusted forwarding contracts
    1: '0x84a0856b038eaAd1cC7E297cF34A7e72685A8693',
    4: '0xFD4973FeB2031D4409fB57afEE5dF2051b171104',
    5: '0xE041608922d06a4F26C0d4c27d8bCD01daf1f792',
    42: '0xF82986F574803dfFd9609BE8b9c7B92f63a1410E',
    100: '0x86C80a8aa58e0A4fa09A69624c31Ab2a6CAD56b8',
    137: '0x86C80a8aa58e0A4fa09A69624c31Ab2a6CAD56b8',
    42161: '0xfe0fa3C06d03bDC7fb49c892BbB39113B534fB57',
    80001: '0x9399BB24DBB5C4b782C70c2969F58716Ebbd6a3b',
    421611: '0x67454E169d613a8e9BA6b06af2D267696EAaAf41'
}

const ForwardRequest = [ // type definition of erc2771 tx forwarding request
    { name: 'from', type: 'address' },
    { name: 'to', type: 'address' },
    { name: 'value', type: 'uint256' },
    { name: 'gas', type: 'uint256' },
    { name: 'nonce', type: 'uint256' },
    { name: 'data', type: 'bytes' },
];

/**
 * Generate network-specific domain seperators
 * @param {string} forwarder - the address to use for the forwarder if known
 * @returns {Object} - the typed domain separator for metatransaction signing
 */
async function getDomain(forwarder) {
    let { chainId } = await ethers.provider.getNetwork()
    return {
        name: 'MinimalForwarder',
        version: '0.0.1',
        chainId,
        verifyingContract: forwarder === undefined ? Forwarders[chainId] : forwarder,
    }
}

/**
 * Add BattleshipGame.sol to biconomy and open metatransaction API
 * @param {string} address - the address of the deployed contract to link to biconomy
 */
async function addContract(address) {
    // get abi of interface for contract to add
    const abi = JSON.stringify(
        require('../../artifacts/contracts/IBattleshipGame.sol/IBattleshipGame.json').abi,
    );
    // reusable http header
    const headers = new URLSearchParams([
        ['authToken', process.env.BICONOMY_AUTH],
        ['apiKey', process.env.BICONOMY_API],
        ['Content-Type', 'application/x-www-form-urlencoded'],
    ]);
    // contract add http body
    let body = new URLSearchParams([
        ['contractName', `BattleZips Contract`],
        ['contractAddress', address],
        ['abi', abi],
        ['contractType', 'SC'],
        ['metaTransactionType', 'TRUSTED_FORWARDER'],
    ]);
    // post to api and add contract to biconomy
    let res = await fetch(
        'https://api.biconomy.io/api/v1/smart-contract/public-api/addContract',
        { method: 'POST', headers, body },
    );
    const methods = ['firstTurn', 'turn', 'joinGame', 'newGame']
    for (const method of methods) {
        // method add http body
        body = new URLSearchParams([
            ['apiType', 'native'],
            ['methodType', 'write'],
            ['name', `BattleZips ${method} function`],
            ['contractAddress', address],
            ['method', method],
        ]);
        // post to api and open method api through contract on biconomy api
        let res = await fetch(
            'https://api.biconomy.io/api/v1/meta-api/public-api/addMethod',
            { method: 'POST', headers, body },
        );
    }
}

/**
 * Create biconomy ethers provider from hardhat-injected ethers
 *
 * @return {Object} - ethers provider capable of using biconomy
 */
async function biconomyProvider() {
    return new Promise((resolve, reject) => {
        // init biconomy objects
        const biconomy = new Biconomy(ethers.provider, {
            apiKey: process.env.BICONOMY_API,
            // debug: true,
            strict: true,
        });
        // wait for connection to establish
        biconomy
            .onEvent(biconomy.READY, () => {
                resolve(biconomy);
            })
            .onEvent(biconomy.ERROR, (err, msg) => {
                console.log('Biconomy Init Error: ', msg);
                reject(err);
            });
    });
}

module.exports = {
    Forwarders,
    ForwardRequest,
    getDomain,
    addContract,
    biconomyProvider
}