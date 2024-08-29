import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day14";

describe("day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  // generate a signer to call our function
  let myKeypair = anchor.web3.Keypair.generate();
  let myKeypair2 = anchor.web3.Keypair.generate();
  let myKeypair3 = anchor.web3.Keypair.generate();

  it("Is signed by a single signer", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
      })
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
  });

  it("Is signed by two signers", async () => {
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

  it("Is signed by three signers", async () => {
    const tx = await program.methods
      .threeSigners()
      .accounts({
        signer1: program.provider.publicKey,
        signer2: myKeypair2.publicKey,
        signer3: myKeypair3.publicKey,
      })
      .signers([myKeypair2, myKeypair3])
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair2.publicKey.toBase58());
    console.log("The signer3: ", myKeypair3.publicKey.toBase58());
  });
});
