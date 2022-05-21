import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CronosDca } from "../target/types/cronos_dca";

describe("cronos-dca", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CronosDca as Program<CronosDca>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
