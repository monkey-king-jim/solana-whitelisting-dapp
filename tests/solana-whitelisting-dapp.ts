import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Whitelisting } from "../target/types/whitelisting";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("solana-whitelisting-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SolanaWhitelistingDapp as Program<Whitelisting>;
  console.log(anchor.workspace);

  it("Is initialized!", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const wallet = provider.wallet;
    const systemProgram = anchor.web3.SystemProgram;
    const program = anchor.workspace.Whitelisting as Program<Whitelisting>;

    const added_address = Keypair.generate();
    const added_address_pubkey = added_address.publicKey;

    const [whitelist, _whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
        program.programId
      );

    const tx = await program.methods
      .createWhitelist("foo")
      .accounts({
        user: wallet.publicKey,
        whitelist,
        systemProgram: systemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
