import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day26 } from "../target/types/day26";

describe("day26", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day26 as Program<Day26>;

  it("Is initialized!", async () => {
    const programId = await program.account.pda.programId;

    let seeds = [];
    let pdaAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      programId
    )[0];

    const tx = await program.methods
      .initialize()
      .accounts({
        pda: pdaAccount,
      })
      .rpc();

    // transfer 2 SOL
    const tx2 = await program.methods
      .donate(new anchor.BN(2_000_000_000))
      .accounts({
        pda: pdaAccount,
      })
      .rpc();

    console.log(
      "lamport balance of pdaAccount",
      await anchor.getProvider().connection.getBalance(pdaAccount)
    );

    // transfer back 1 SOL
    // the signer is the permitted address
    await program.methods
      .withdraw(new anchor.BN(1_000_000_000))
      .accounts({
        pda: pdaAccount,
      })
      .rpc();

    console.log(
      "lamport balance of pdaAccount",
      await anchor.getProvider().connection.getBalance(pdaAccount)
    );
  });
});
