import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day29 } from "../target/types/day29";

describe("day29", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day29 as Program<Day29>;

  it("Is initialized!", async () => {
    const programId = await program.account.myStorage.programId;

    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    // Initialize the storage.
    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
    console.log("Account initialized successfully!");
  });
});
