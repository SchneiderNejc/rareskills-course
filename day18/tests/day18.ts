import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day18 } from "../target/types/day18";

describe("day18", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day18 as Program<Day18>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myStorage.toBase58());

    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
    await program.methods
      .set(new anchor.BN(170))
      .accounts({ myStorage: myStorage })
      .rpc();

    // This method only works for accounts your program has created and have the IDL for.
    let myStorageStruct = await program.account.myStorage.fetch(myStorage);
    console.log("The value of x is:", myStorageStruct.x.toString());
  });
});
