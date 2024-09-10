import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day24";

// ------------------- Helper functions -------------------
// this airdrops sol to an address
// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

// ------------------- Test functions -------------------
describe("day24", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day24 as Program<Day24>;

  it("Alice transfers points to Bob", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();

    const airdrop_alice_tx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdrop_alice_tx);

    const airdrop_alice_bob = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdrop_alice_bob);

    let seeds_alice = [alice.publicKey.toBytes()];
    const [playerAlice, _bumpA] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_alice,
      program.programId
    );

    let seeds_bob = [bob.publicKey.toBytes()];
    const [playerBob, _bumpB] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds_bob,
      program.programId
    );

    // Alice and Bob initialize their accounts
    await program.methods
      .initialize()
      .accounts({
        player: playerAlice,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize()
      .accounts({
        player: playerBob,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    // Alice transfers 5 points to Bob. Note that this is a u32
    // so we don't need a BigNum
    await program.methods
      .transferPoints(5)
      .accounts({
        from: playerAlice,
        to: playerBob,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    console.log(
      `Alice has ${
        (await program.account.player.fetch(playerAlice)).points
      } points`
    );
    console.log(
      `Bob has ${(await program.account.player.fetch(playerBob)).points} points`
    );
  });
  });
});
