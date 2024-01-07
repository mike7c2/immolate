import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Immolate } from "../target/types/immolate";

describe("immolate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Immolate as Program<Immolate>;

  it("Is initialized!", async () => {
    program.methods.immolate()
    // Add your test here.
    const tx = await program.methods.immolate().accounts({
      mint: "",
      tokenProgram: "",
      from: "",
      authority: ""
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
