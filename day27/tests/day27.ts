import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day27 } from "../target/types/day27";

describe("day27", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day27 as Program<Day27>;

  it("initialize after giving to system program or draining lamports", async () => {
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    await program.methods.initialize().accounts({ myPda: myPda }).rpc();

    await program.methods
      .giveToSystemProgram()
      .accounts({ myPda: myPda })
      .rpc();

    await program.methods.initialize().accounts({ myPda: myPda }).rpc();
    console.log("account initialized after giving to system program!");

    await program.methods.drainLamports().accounts({ myPda: myPda }).rpc();

    await program.methods.initialize().accounts({ myPda: myPda }).rpc();
    console.log("account initialized after draining lamports!");
  });
});
