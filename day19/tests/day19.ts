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
    const key = new anchor.BN(42);
    const value = new anchor.BN(1337);

    const seeds = [key.toArrayLike(Buffer, "le", 8)];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods.initialize(key).accounts({ val: valueAccount }).rpc();

    // set the account
    await program.methods.set(key, value).accounts({ val: valueAccount }).rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);

    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );
  });
});
