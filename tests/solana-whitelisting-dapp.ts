import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Whitelisting } from "../target/types/whitelisting";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("solana-whitelisting-dapp", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet;
  const systemProgram = anchor.web3.SystemProgram;
  const program = anchor.workspace.Whitelisting as Program<Whitelisting>;
  const add_to_whitelist = Keypair.generate();

  // console.log(anchor.workspace);

  it("Is initialized!", async () => {
    const [whitelist, _whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
        program.programId
      );

    console.log(typeof whitelist);

    // const tx = await program.methods
    //   .createWhitelist("test")
    //   .accounts({
    //     authority: wallet.publicKey,
    //     whitelist,
    //     systemProgram: systemProgram.programId,
    //   })
    //   .rpc();
    // console.log("Your transaction signature", tx);
  });

  it("Fetched the base whitelist!", async () => {
    const [whitelistPubkey, _whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
        program.programId
      );

    const whitelist = await program.account.whitelist.fetch(whitelistPubkey);
    console.log("Your whitelist", whitelist);
  });

  it("Added a wallet to the whitelist!", async () => {
    const [whitelistPubkey, _whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
        program.programId
      );

    const [whitelistDataPubkey, _whitelistDataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [whitelistPubkey.toBytes(), add_to_whitelist.publicKey.toBytes()],
        program.programId
      );

    const tx = await program.methods
      .addToWhitelist()
      .accounts({
        authority: wallet.publicKey,
        whitelist: whitelistPubkey,
        wallet: add_to_whitelist.publicKey,
        whitelistData: whitelistDataPubkey,
        systemProgram: systemProgram.programId,
      })
      .rpc();
    //   // const latestBlockHash = await provider.connection.getLatestBlockhash();
    //   // await provider.connection.confirmTransaction({
    //   //   blockhash: latestBlockHash.blockhash,
    //   //   lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    //   //   signature: tx,
    //   // });
    // });

    it("Verified a whitelist data!", async () => {
      const [whitelistPubkey, _whitelistBump] =
        await anchor.web3.PublicKey.findProgramAddress(
          [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
          program.programId
        );

      const [whitelistDataPubkey, _whitelistDataBump] =
        await anchor.web3.PublicKey.findProgramAddress(
          [whitelistPubkey.toBytes(), add_to_whitelist.publicKey.toBytes()],
          program.programId
        );

      const whitelistData = await program.account.whitelistData.fetch(
        whitelistDataPubkey
      );
      // console.log("whitelistDataPubkey", whitelistDataPubkey.toString());
      console.log("The data is whitelisted: ", whitelistData);
    });
  });
});
