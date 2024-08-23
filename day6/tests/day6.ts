import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day6";

describe("day6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Age checker", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Prints struct.", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.structure("Paul", new anchor.BN(64)).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Prints mapping!", async () => {
    // Add your test here.
    const tx = await program.methods.mapping("name", "Bob").rpc();
  // it("Cast array length.", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.castArrayLength().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Filter even numbers from the array.", async () => {
    // Define the input vector of numbers
    const inputVector = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map(
      (n) => new anchor.BN(n)
    );

    // Call the `filter_even_numbers` method on the program
    const tx = await program.methods.filterEvenNumbers(inputVector).rpc();
    console.log("Your transaction signature", tx);

    // Check if the even numbers were filtered correctly:
    const expectedEvenNumbers = [2, 4, 6, 8, 10].map((n) => new anchor.BN(n));
    console.log("Expected even numbers:", expectedEvenNumbers);
  });
});
