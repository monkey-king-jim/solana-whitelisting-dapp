import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaWhitelistingDapp } from "../target/types/solana_whitelisting_dapp";

describe("solana-whitelisting-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaWhitelistingDapp as Program<SolanaWhitelistingDapp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
