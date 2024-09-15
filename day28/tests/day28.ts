import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day28 } from "../target/types/day28";

describe("day28", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day28 as Program<Day28>;

  it("Set the number to 5, initializing if necessary", async () => {
    const wallet = anchor.workspace.Day28.provider.wallet.payer;
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    // console.log the address of the pda
    console.log(pda.toBase58());

    let transaction = new anchor.web3.Transaction();
    transaction.add(
      await program.methods.initialize().accounts({ pda: pda }).transaction()
    );
    transaction.add(
      await program.methods.set(5).accounts({ pda: pda }).transaction()
    );

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [wallet]
    );
  });
});
