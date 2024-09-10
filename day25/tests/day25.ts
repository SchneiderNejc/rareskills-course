import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day25 } from "../target/types/day25";

describe("day25", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day25 as Program<Day25>;

  it("Is initialized -- PDA version", async () => {
    const seeds = [];
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("the storage account address is", myPda.toBase58());

    const tx = await program.methods
      .initializePda()
      .accounts({ myPda: myPda })
      .rpc();
  });
});
