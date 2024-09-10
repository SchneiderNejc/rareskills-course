import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day25 } from "../target/types/day25";

// --------------------- Helper Functions ---------------------
// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(
      publicKey,
      amount * anchor.web3.LAMPORTS_PER_SOL
    );
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

// --------------------- Test Functions ---------------------
describe("day25", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day25 as Program<Day25>;

  // @notice Will only succeed the first time.
  it("Is initialized -- PDA version", async () => {
    const seeds = [];
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myPda.toBase58());

    const tx = await program.methods
      .initializePda()
      .accounts({ myPda: myPda })
      .rpc();
  });

  it("Is initialized -- keypair version", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await airdropSol(newKeypair.publicKey, 1e9); // 1 SOL

    console.log(
      "the keypair account address is",
      newKeypair.publicKey.toBase58()
    );

    await program.methods
      .initializeKeypairAccount()
      .accounts({ myKeypairAccount: newKeypair.publicKey })
      .signers([newKeypair]) // the signer must be the keypair
      .rpc();
  });
});
