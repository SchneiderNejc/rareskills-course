import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day6";

describe("day6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Age checker", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Prints mapping!", async () => {
    // Add your test here.
    const tx = await program.methods.mapping("name", "Bob").rpc();
    console.log("Your transaction signature", tx);
  });
});
