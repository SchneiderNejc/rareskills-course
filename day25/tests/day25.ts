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
  xit("Is initialized -- PDA version.", async () => {
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

  it("Is initialized -- keypair version.", async () => {
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

  // @notice Expects fail on second transfer tx.
  xit("Writing to keypair account fails.", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    var recieverWallet = anchor.web3.Keypair.generate();

    await airdropSol(newKeypair.publicKey, 10);

    // First TX succeeds.
    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: recieverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [newKeypair]
    );
    console.log("sent 1 lamport");

    // Init account and program becomes its owner.
    await program.methods
      .initializeKeypairAccount()
      .accounts({ myKeypairAccount: newKeypair.publicKey })
      .signers([newKeypair]) // the signer must be the keypair
      .rpc();

    console.log("initialized.");

    // PK no longer owns an account, so the tx fails.
    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: recieverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [newKeypair]
    );
  });

  it("Console log account owner.", async () => {
    console.log(`The program address is ${program.programId}`);
    const newKeypair = anchor.web3.Keypair.generate();

    // get account owner before initialization
    await airdropSol(newKeypair.publicKey, 10);
    const accountInfoBefore = await anchor
      .getProvider()
      .connection.getAccountInfo(newKeypair.publicKey);
    console.log(`initial keypair account owner is ${accountInfoBefore.owner}`);

    await program.methods
      .initializeKeypairAccount()
      .accounts({ myKeypairAccount: newKeypair.publicKey })
      .signers([newKeypair]) // the signer must be the keypair
      .rpc();

    // get account owner after initialization
    const accountInfoAfter = await anchor
      .getProvider()
      .connection.getAccountInfo(newKeypair.publicKey);
    console.log(
      `keypair account owner after init is ${accountInfoAfter.owner}`
    );
  });
});
