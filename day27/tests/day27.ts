import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day27 } from "../target/types/day27";

describe("day27", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day27 as Program<Day27>;

  // @notice Expected to fail.
  it("Can't initialize after erasing the account.", async () => {
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );

    await program.methods.initialize().accounts({ myPda: myPda }).rpc();

    await program.methods.erase().accounts({ myPda: myPda }).rpc();

    await program.methods.initialize().accounts({ myPda: myPda }).rpc();
    console.log("account initialized after giving to system program!");
  });
});
