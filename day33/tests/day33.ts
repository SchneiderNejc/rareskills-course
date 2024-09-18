import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day33 } from "../target/types/day33";
import { Alice } from "../target/types/alice";

describe("CPI from Alice to Day33", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Day33 as Program<Day33>;
  const aliceProgram = anchor.workspace.Alice as Program<Alice>;
  const dataAccountKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])
      .rpc();
  });

  it("Can add two numbers.", async () => {
    // Add your test here.
    const tx = await aliceProgram.methods
      .askDay33ToAdd(new anchor.BN(4), new anchor.BN(2))
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        day33Program: program.programId,
      })
      .rpc();
  });
  });
});
