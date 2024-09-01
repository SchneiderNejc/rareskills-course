import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day16 } from "../target/types/day16";

describe("day16", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day16 as Program<Day16>;

  it("Is initialized!", async () => {
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
});
