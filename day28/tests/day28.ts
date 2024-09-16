import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day28 } from "../target/types/day28";

describe("day28", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day28 as Program<Day28>;

  // @notice Expect fail. Batched tx's are atomic,
  //         if second call fails, first one will be reverted.
  it("Set the number to 5, initializing if necessary", async () => {
    const wallet = anchor.workspace.Day28.provider.wallet.payer;
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    let accountInfo = await anchor.getProvider().connection.getAccountInfo(pda);

    let transaction = new anchor.web3.Transaction();
    if (
      accountInfo == null ||
      accountInfo.lamports == 0 ||
      accountInfo.owner == anchor.web3.SystemProgram.programId
    ) {
      console.log("need to initialize");
      const initTx = await program.methods
        .initialize()
        .accounts({ pda: pda })
        .transaction();
      transaction.add(initTx);
    } else {
      console.log("no need to initialize");
    }

    // we're going to set the number anyway
    const setTx = await program.methods
      .set(5)
      .accounts({ pda: pda })
      .transaction();
    transaction.add(setTx);

    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [wallet]
    );

    const pdaAcc = await program.account.pda.fetch(pda);
    console.log(pdaAcc.value);
  });
});
