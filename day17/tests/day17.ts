import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day17 } from "../target/types/day17";

describe("day17", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day17 as Program<Day17>;

  // ------------------- Helper functions -------------------
  function getMyStorageAddress() {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("The storage account address is", myStorage.toBase58());

    return myStorage;
  }

  // ------------------- Tests -------------------
  // @dev Only call once. Follow-up runs will fail
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
    const myStorage = getMyStorageAddress();

    // Set value.
    await program.methods
      .set(new anchor.BN(170))
      .accounts({ myStorage: myStorage })
      .rpc();

    // Print value.
    await program.methods.printX().accounts({ myStorage: myStorage }).rpc();
  });

  it("Can get value.", async () => {
    const myStorage = getMyStorageAddress();

    // Slow way of parsing
    /*     let x = await program.methods
      .get()
      .accounts({ myStorage: myStorage })
      .view();
    x = new anchor.BN(x).toNumber(); */
    // Fast way of parsing
    let x = (
      await program.methods.get().accounts({ myStorage }).view()
    ).toNumber();
    console.log("val of x is", x);

    // @todo assert get value is 170
  });

  it("Can increment value.", async () => {
    const myStorage = getMyStorageAddress();

    // Set incremented value.
    await program.methods.increment().accounts({ myStorage: myStorage }).rpc();

    await program.methods.printX().accounts({ myStorage: myStorage }).rpc();
  });
  });
});
