require('dotenv').config()
const { ethers } = require('hardhat')
const snarkjs = require('snarkjs')
const { boards, verificationKeys, initialize, buildProofArgs, biconomy } = require("./utils")
const { solidity } = require("ethereum-waffle");
const chai = require("chai").use(solidity)

describe("Metatransaction integration testing", () => {
    let operator, alice, bob // players
    let game, forwarder // contracts
    let F // ffjavascript BN254 construct
    let boardHashes // store hashed board for alice and bob

    before(async () => {
        // set players
        const signers = await ethers.getSigners()
        operator = signers[0];
        alice = signers[1];
        bob = signers[2];
        // initialize and store 
        ({ game, F, boardHashes, forwarder } = await initialize(ethers.constants.AddressZero));
    })
    it('Perform a snark action through meta-transaction signature + relay', async () => {
        // board starting verification proof public / private inputs
        const input = {
            ships: boards.alice,
            hash: F.toObject(boardHashes.alice)
        }
        // compute witness and run through groth16 circuit for proof / signals
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input,
            'zk/board_js/board.wasm',
            'zk/zkey/board_final.zkey',
        )
        // verify proof locally
        await snarkjs.groth16.verify(
            verificationKeys.board,
            publicSignals,
            proof
        )
        // prove on-chain hash is of valid board configuration
        const proofArgs = buildProofArgs(proof, publicSignals)
        // create metatransaction
        const { data } = await game.populateTransaction.newGame(
            F.toObject(boardHashes.alice),
            ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
        )
        const minParams = { // params needed to estimate gas used in tx
            to: game.address,
            from: alice.address,
            data,
        }
        const gas = await ethers.provider.estimateGas(minParams)
        const msg = { // params to define a transaction to be signed
            gas,
            value: ethers.BigNumber.from(0),
            nonce: ethers.BigNumber.from(0),
            ...minParams
        }
        const domain = await biconomy.getDomain(forwarder.address)
        // sign transaction
        const signature = await alice._signTypedData(
            domain,
            { ForwardRequest: biconomy.ForwardRequest },
            msg
        )
        // verify signature locally
        const verifiedAddress = await ethers.utils.verifyTypedData( 
            domain,
            { ForwardRequest: biconomy.ForwardRequest },
            msg,
            signature,
        )
        chai.expect(alice.address).to.be.equal(verifiedAddress)
        // establish base state in game
        let playing = await game.playing(alice.address)
        chai.expect(playing).to.be.equal(ethers.BigNumber.from(0))
        // execute transaction as metatransaction through trusted relay
        await (await forwarder.connect(operator).execute(msg, signature)).wait()
        // check for game state advancement
        playing = await game.playing(alice.address)
        chai.expect(playing).to.be.equal(ethers.BigNumber.from(1))
    })

})