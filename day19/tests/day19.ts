import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day19 } from "../target/types/day19";

describe("day19", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day19 as Program<Day19>;

  it("Initialize mapping storage", async () => {
    const key = new anchor.BN(42);

    const seeds = [key.toArrayLike(Buffer, "le", 8)];
    const [valueAccount, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods.initialize(key).accounts({ val: valueAccount }).rpc();
  });
});
