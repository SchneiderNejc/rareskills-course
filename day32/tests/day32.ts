import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day32 } from "../target/types/day32";

describe("day32", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day32 as Program<Day32>;

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
