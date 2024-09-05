import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day20 } from "../target/types/day20";

describe("day20", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day20 as Program<Day20>;

  // ------------------- Helper functions -------------------
  // @todo store myStorage in global var and only fetch and print it once.
  function getMyStorageAddress() {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("The storage account address is", myStorage.toBase58());

    return myStorage;
  }

  // ------------------- Tests -------------------
  // @note Only run once.
  xit("Is initialized!", async () => {
    const myStorage = getMyStorageAddress();

    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
  });

  // @notice Verify account size with solana account [address]
  it("Increased account size.", async () => {
    const myStorage = getMyStorageAddress();

    await program.methods
      .increaseAccountSize()
      .accounts({ myStorage: myStorage })
      .rpc();
  });
});
