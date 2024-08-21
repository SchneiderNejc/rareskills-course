import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day3 } from "../target/types/day3";

describe("day3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day3 as Program<Day3>;

  it("Call boaty mcboatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should add", async () => {
    const tx = await program.methods
      .add(new anchor.BN(1), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should sub", async () => {
    const tx = await program.methods
      .sub(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should mul", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should div", async () => {
    const tx = await program.methods
      .div(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should modulus", async () => {
    const tx = await program.methods
      .modulus(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
