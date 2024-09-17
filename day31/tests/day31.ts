import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day31 } from "../target/types/day31";

describe("day31", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day31 as Program<Day31>;

  it("Wrong owner with Account", async () => {
    await program.methods.hello().rpc();
  });
});
