import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day26 } from "../target/types/day26";

import privateKey from "/home/nejc/.config/solana/id.json";
// ------------------ Helper functions ------------------
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

// ------------------ Test functions ------------------
// @notice init can only be run once, further runs will fail.
describe("day26", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day26 as Program<Day26>;

  it("Is initialized!", async () => {
    console.log("program address", program.programId.toBase58());

    // ------------------ PDA Section ------------------
    const seeds = [];
    const [pda, bump_] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log(
      "owner of pda before airdrop:",
      await anchor.getProvider().connection.getAccountInfo(pda)
    );

    await airdropSol(pda, 1); // 1 SOL

    console.log(
      "owner of pda before airdrop:",
      (
        await anchor.getProvider().connection.getAccountInfo(pda)
      ).owner.toBase58()
    );

    await program.methods.initializePda().accounts({ pda: pda }).rpc();

    console.log(
      "owner of pda after initialize:",
      (
        await anchor.getProvider().connection.getAccountInfo(pda)
      ).owner.toBase58()
    );

    // ------------------ Keypair Section ------------------
    let keypair = anchor.web3.Keypair.generate();

    console.log(
      "owner of keypair before airdrop:",
      await anchor.getProvider().connection.getAccountInfo(keypair.publicKey)
    );

    await airdropSol(keypair.publicKey, 1); // 1 SOL

    console.log(
      "owner of keypair after airdrop:",
      (
        await anchor.getProvider().connection.getAccountInfo(keypair.publicKey)
      ).owner.toBase58()
    );

    await program.methods
      .initializeKeypair()
      .accounts({ keypair: keypair.publicKey })
      .signers([keypair]) // the signer must be the keypair
      .rpc();

    console.log(
      "owner of keypair after initialize:",
      (
        await anchor.getProvider().connection.getAccountInfo(keypair.publicKey)
      ).owner.toBase58()
    );

    console.log("PDA address:", pda.toBase58());
    console.log("Keypair address:", keypair.publicKey.toBase58());
  });

  it("Is initialized!", async () => {
    const deployer = anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from(privateKey)
    );

    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myStorage.toBase58());

    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
    await program.methods
      .changeOwner()
      .accounts({ myStorage: myStorage })
      .rpc();

    // after the ownership has been transferred
    // the account can still be initialized again
    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
  });
});
