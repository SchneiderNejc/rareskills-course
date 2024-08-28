import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day14";

describe("day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  it("Is signed by a single signer", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
      })
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
  });

  it("Is signed by multiple signers", async () => {
    // generate a signer to call our function
    let myKeypair = anchor.web3.Keypair.generate();

    // Add your test here.
    const tx = await program.methods
      .twoSigners()
      .accounts({
        signer1: program.provider.publicKey,
        signer2: myKeypair.publicKey,
      })
      .signers([myKeypair])
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
  });
});
