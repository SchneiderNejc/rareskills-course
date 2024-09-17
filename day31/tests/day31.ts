import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day31 } from "../target/types/day31";

describe("day31", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day31 as Program<Day31>;

  // ----------------- Helper functions -----------------
  async function airdropSol(publicKey, amount) {
    let airdropTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        publicKey,
        amount * anchor.web3.LAMPORTS_PER_SOL
      );

    await confirmTransaction(airdropTx);
  }

  async function confirmTransaction(tx) {
    const latestBlockHash = await anchor
      .getProvider()
      .connection.getLatestBlockhash();

    await anchor.getProvider().connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  }

  // ----------------- Test functions -----------------

  // @notice Expects to fail. Account is not owned by program
  it("Wrong owner with Account", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await airdropSol(newKeypair.publicKey, 10);

    await program.methods
      .foo()
      .accounts({ someAccount: newKeypair.publicKey })
      .rpc();
  });
});
