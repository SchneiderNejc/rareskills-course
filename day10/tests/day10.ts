import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { day10 } from "../target/types/day10";

describe("day10", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day10 as Program<day10>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
