import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Got } from "../target/types/got";

describe("got", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Got as Program<Got>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize2().rpc();
    console.log("Your transaction signature", tx);
  });
});
