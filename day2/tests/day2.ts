import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(777), new anchor.BN(888), "hello")
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods
      .array([new anchor.BN(777), new anchor.BN(888)])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // --------------- MATH TESTS ---------------

  // Constants
  const a: number = 9;
  const b: number = 2;

  it("add", async () => {
    const tx = await program.methods
      .add(new anchor.BN(a), new anchor.BN(b))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("subtract", async () => {
    const tx = await program.methods
      .subtract(new anchor.BN(a), new anchor.BN(b))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("multiply", async () => {
    const tx = await program.methods
      .multiply(new anchor.BN(a), new anchor.BN(b))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("divide", async () => {
    const tx = await program.methods.divide(a, b).rpc();
    console.log("Your transaction signature", tx);
  });

  it("square root", async () => {
    const tx = await program.methods.sqrt(a).rpc();
    console.log("Your transaction signature", tx);
  });

  it("log 10", async () => {
    const tx = await program.methods.log10(a).rpc();
    console.log("Your transaction signature", tx);
  });
});
