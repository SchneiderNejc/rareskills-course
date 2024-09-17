import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day31 } from "../target/types/day31";

describe("day31", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day31 as Program<Day31>;
  const wallet = anchor.workspace.Day31.provider.wallet;

  it("Load account with accountInfo", async () => {
    // CREATE AN ACCOUNT NOT OWNED BY THE PROGRAM
    const newKeypair = anchor.web3.Keypair.generate();
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: newKeypair.publicKey,
        space: 16,
        lamports: await anchor
          .getProvider()
          .connection.getMinimumBalanceForRentExemption(32),
        programId: program.programId,
      })
    );

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      tx,
      [wallet.payer, newKeypair]
    );

    // READ THE DATA IN THE ACCOUNT
    await program.methods
      .foo()
      .accounts({ someAccount: newKeypair.publicKey })
      .rpc();
  });
});
