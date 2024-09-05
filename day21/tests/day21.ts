import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day21 } from "../target/types/day21";

describe("day21", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day21 as Program<Day21>;

  // the following is the Solana wallet we are using
  let pubkey = new anchor.web3.PublicKey(
    "6ZJDfSVjffvRYbtpFF33PSeYWNYJnbtMKiwWVt1YRjZW"
  );

  it("Tests the balance", async () => {
    const tx = await program.methods
      .readBalance()
      .accounts({ acct: pubkey })
      .rpc();
  });
});
