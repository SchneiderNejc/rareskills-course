import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day17 } from "../target/types/day17";

describe("day17", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day17 as Program<Day17>;

  xit("Is initialized!", async () => {
    // Calculate account address that stores MyStruct.
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myStorage.toBase58());

    // Pass address to initialize().
    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
  });

  it("Can set value.", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myStorage.toBase58());

    await program.methods
      .set(new anchor.BN(170))
      .accounts({ myStorage: myStorage })
      .rpc();
  it("Can increment value.", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    // Set incremented value.
    await program.methods.increment().accounts({ myStorage: myStorage }).rpc();

    await program.methods.printX().accounts({ myStorage: myStorage }).rpc();
  });
  });
});
