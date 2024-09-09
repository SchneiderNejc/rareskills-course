import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day24";

// ------------------- Helper functions -------------------
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

  it("Is initialized!", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await airdropSol(newKeypair.publicKey, 1e9); // 1 SOL

    let seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        myStorage: myStorage,
        signer: newKeypair.publicKey, // ** THIS MUST BE EXPLICITLY SPECIFIED **
      })
      .signers([newKeypair])
      .rpc();
  });
});
