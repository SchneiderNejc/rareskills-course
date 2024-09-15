import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day28 } from "../target/types/day28";

describe("day28", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day28 as Program<Day28>;

  it("Is initialized!", async () => {
    const wallet = anchor.workspace.Day28.provider.wallet.payer;
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    const initTx = await program.methods
      .initialize()
      .accounts({ pda: pda })
      .transaction();

    // for u32, we don't need to use big numbers
    const setTx = await program.methods
      .set(5)
      .accounts({ pda: pda })
      .transaction();

    let transaction = new anchor.web3.Transaction();
    transaction.add(initTx);
    transaction.add(setTx);

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [wallet]
    );

    const pdaAcc = await program.account.pda.fetch(pda);
    console.log(pdaAcc.value); // prints 5
  });
});
