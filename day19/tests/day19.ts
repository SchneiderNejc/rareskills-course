import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day19 } from "../target/types/day19";

describe("day19", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day19 as Program<Day19>;

  // @dev Will only work once. For consequtive tests
  // restart the client with solana-test-validator --reset
  it("Initialize and set value", async () => {
    // we now have two keys
    const key1 = new anchor.BN(42);
    const key2 = new anchor.BN(43);
    const key = new anchor.BN(44);
    const value = new anchor.BN(1337);

    // Seeds
    const seedsMapping1 = [
      Buffer.from("mapping1"), // Distinct identifier for the first mapping
      key1.toArrayLike(Buffer, "le", 8),
      key2.toArrayLike(Buffer, "le", 8),
    ];
    const seedsMapping2 = [
      Buffer.from("mapping2"), // Distinct identifier for the second mapping
      key.toArrayLike(Buffer, "le", 8),
    ];
    // Values
    let valueAccountMapping1 = anchor.web3.PublicKey.findProgramAddressSync(
      seedsMapping1,
      program.programId
    )[0];

    let valueAccountMapping2 = anchor.web3.PublicKey.findProgramAddressSync(
      seedsMapping2,
      program.programId
    )[0];

    await program.methods
      .initialize(key1, key2)
      .accounts({ val: valueAccountMapping1 })
      .rpc();

    await program.methods
      .initializeSecond(key)
      .accounts({ val: valueAccountMapping2 })
      .rpc();

    await program.methods
      .set(key1, key2, value)
      .accounts({ val: valueAccountMapping1 })
      .rpc();

    await program.methods
      .setSecond(key, value)
      .accounts({ val: valueAccountMapping2 })
      .rpc();

    // read the account back for mapping1
    let resultMapping1 = await program.account.val.fetch(valueAccountMapping1);
    console.log(
      `the value ${
        resultMapping1.value
      } was stored in ${valueAccountMapping1.toBase58()}`
    );

    // read the account back for mapping2
    let resultMapping2 = await program.account.val.fetch(valueAccountMapping2);
    console.log(
      `the value ${
        resultMapping2.value
      } was stored in ${valueAccountMapping2.toBase58()}`
    );
  });
});
