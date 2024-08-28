import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13 } from "../target/types/day13";

describe("day13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day13 as Program<Day13>;

  it("Is initialized!", async () => {
    const listenerMyEvent = program.addEventListener(
      "MyEvent",
      (event, slot) => {
        console.log(`slot ${slot} event value ${event.value}`);
      }
    );

    const listenerMySecondEvent = program.addEventListener(
      "MySecondEvent",
      (event, slot) => {
        console.log(
          `slot ${slot} event value ${event.value} event message ${event.message}`
        );
      }
    );

    const tx = await program.methods.initialize().rpc();

    // This line is only for test purposes to ensure the event
    // listener has time to listen to event.
    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listenerMyEvent);
    program.removeEventListener(listenerMySecondEvent);
  });
});
