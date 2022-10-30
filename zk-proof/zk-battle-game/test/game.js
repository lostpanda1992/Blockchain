const { ethers } = require('hardhat')
const snarkjs = require('snarkjs')
const {
    boards,
    shots,
    verificationKeys,
    initialize,
    buildProofArgs,
    printLog,
} = require("./utils")

describe('Play Battleship on-chain', async () => {
    let operator, alice, bob // players
    let game // contracts
    let F // ffjavascript BN254 construct
    let boardHashes // store hashed board for alice and bob

    /**
     * Simulate one guaranteed hit and one guaranteed miss played in the game
     * 
     * @param aliceNonce number - number of shots alice has already taken
     *  - range should be 1 through 16 for structured test
     */
    async function simulateTurn(aliceNonce) {
        printLog(`Bob reporting result of Alice shot #${aliceNonce - 1} (Turn ${aliceNonce * 2 - 1})`)
        /// BOB PROVES ALICE PREV REGISTERED SHOT HIT ///
        // bob's shot hit/miss integrity proof public / private inputs
        let input = {
            ships: boards.bob,
            hash: F.toObject(boardHashes.bob),
            shot: shots.alice[aliceNonce - 1],
            hit: 1,
        }
        // compute witness and run through groth16 circuit for proof / signals
        let { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input,
            'zk/shot_js/shot.wasm',
            'zk/zkey/shot_final.zkey'
        )
        // verify proof locally
        await snarkjs.groth16.verify(verificationKeys.shot, publicSignals, proof)
        // prove alice's registered shot hit, and register bob's next shot
        let proofArgs = buildProofArgs(proof)
        tx = await (await game.connect(bob).turn(
            1, // game id
            true, // hit bool
            shots.bob[aliceNonce - 1], // returning fire / next shot to register (not part of proof)
            ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
        )).wait()
        /// ALICE PROVES BOB PREV REGISTERED SHOT MISSED ///
        printLog(`Alice reporting result of Bob shot #${aliceNonce - 1} (Turn ${aliceNonce * 2})`)
        // bob's shot hit/miss integrity proof public / private inputs
        input = {
            ships: boards.alice,
            hash: F.toObject(boardHashes.alice),
            shot: shots.bob[aliceNonce - 1],
            hit: 0
        };
        // compute witness and run through groth16 circuit for proof / signals
        ({ proof, publicSignals } = await snarkjs.groth16.fullProve(
            input,
            'zk/shot_js/shot.wasm',
            'zk/zkey/shot_final.zkey'
        ));
        // verify proof locally
        await snarkjs.groth16.verify(verificationKeys.shot, publicSignals, proof)
        // prove bob's registered shot missed, and register alice's next shot
        proofArgs = buildProofArgs(proof)
        await (await game.connect(alice).turn(
            1, // game id
            false, // hit bool
            shots.alice[aliceNonce], // returning fire / next shot to register (not part of proof)
            ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
        )).wait()
    }

    before(async () => {
        // set players
        const signers = await ethers.getSigners()
        operator = signers[0];
        alice = signers[1];
        bob = signers[2];
        // initialize and store 
        ({ bv, sv, game, F, boardHashes } = await initialize(ethers.constants.AddressZero))
    })

    describe("Play game to completion", async () => {
        it("Start a new game", async () => {
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
            const proofArgs = buildProofArgs(proof)
            let tx = await (await game.connect(alice).newGame(
                F.toObject(boardHashes.alice),
                ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
            )).wait()
        })
        it("Join an existing game", async () => {
            // board starting verification proof public / private inputs
            const input = {
                ships: boards.bob,
                hash: F.toObject(boardHashes.bob)
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
            const proofArgs = buildProofArgs(proof)
            await (await game.connect(bob).joinGame(
                1,
                F.toObject(boardHashes.bob),
                ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
            ))
        })
        it("opening shot", async () => {
            await (await game.connect(alice).firstTurn(1, [1, 0])).wait()
        })
        it('Prove hit/ miss for 32 turns', async () => {
            for (let i = 1; i <= 16; i++) {
                await simulateTurn(i)
            }
        })
        it('Alice wins on sinking all of Bob\'s ships', async () => {
            // bob's shot hit/miss integrity proof public / private inputs
            const input = {
                ships: boards.bob,
                hash: F.toObject(boardHashes.bob),
                shot: shots.alice[16],
                hit: 1
            }
            // compute witness and run through groth16 circuit for proof / signals
            const { proof, publicSignals } = await snarkjs.groth16.fullProve(
                input,
                'zk/shot_js/shot.wasm',
                'zk/zkey/shot_final.zkey'
            )
            // verify proof locally
            await snarkjs.groth16.verify(verificationKeys.shot, publicSignals, proof)
            // prove alice's registered shot hit, and register bob's next shot
            const proofArgs = buildProofArgs(proof)
            await (await game.connect(bob).turn(
                1, // game id
                true, // hit bool
                [0, 0], // shot params are ignored on reporting all ships sunk, can be any uint256
                ...proofArgs //pi_a, pi_b_0, pi_b_1, pi_c
            )).wait()
        })
    })
})
