import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter } from "../target/types/counter";
import { Whitelisting } from "../target/types/whitelisting";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("counter-test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet;
  const systemProgram = anchor.web3.SystemProgram;
  const whitelist_program = anchor.workspace
    .Whitelisting as Program<Whitelisting>;
  const counter_program = anchor.workspace.Counter as Program<Counter>;
  // console.log(anchor.workspace);

  it("Is initialized!", async () => {
    const [counter, _] = await anchor.web3.PublicKey.findProgramAddress(
      [wallet.publicKey.toBytes()],
      counter_program.programId
    );

    const [whitelist, _whitelistBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("counter"), wallet.publicKey.toBytes()],
        whitelist_program.programId
      );

    const tx = await counter_program.methods
      .createCounter()
      .accounts({
        counter,
        whitelistConfig: whitelist,
        whitelistingProgram: whitelist_program.programId,
        systemProgram: systemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Fetched the base whitelist!", async () => {
  //   const [whitelistPubkey, _whitelistBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
  //       program.programId
  //     );

  //   const whitelist = await program.account.whitelist.fetch(whitelistPubkey);
  //   console.log("Your whitelist", whitelist);
  // });

  // it("Added a wallet to the whitelist!", async () => {
  //   const [whitelistPubkey, _whitelistBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
  //       program.programId
  //     );

  //   const [whitelistDataPubkey, _whitelistDataBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [whitelistPubkey.toBytes(), add_to_whitelist.publicKey.toBytes()],
  //       program.programId
  //     );

  //   const tx = await program.methods
  //     .addToWhitelist()
  //     .accounts({
  //       authority: wallet.publicKey,
  //       whitelist: whitelistPubkey,
  //       wallet: add_to_whitelist.publicKey,
  //       whitelistData: whitelistDataPubkey,
  //       systemProgram: systemProgram.programId,
  //     })
  //     .rpc();
  //   // const latestBlockHash = await provider.connection.getLatestBlockhash();
  //   // await provider.connection.confirmTransaction({
  //   //   blockhash: latestBlockHash.blockhash,
  //   //   lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
  //   //   signature: tx,
  //   // });
  // });

  // it("Verified a whitelist data!", async () => {
  //   const [whitelistPubkey, _whitelistBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [anchor.utils.bytes.utf8.encode("test"), wallet.publicKey.toBytes()],
  //       program.programId
  //     );

  //   const [whitelistDataPubkey, _whitelistDataBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [whitelistPubkey.toBytes(), add_to_whitelist.publicKey.toBytes()],
  //       program.programId
  //     );

  //   const whitelistData = await program.account.whitelistData.fetch(
  //     whitelistDataPubkey
  //   );
  //   // console.log("whitelistDataPubkey", whitelistDataPubkey.toString());
  //   console.log("The data is whitelisted: ", whitelistData);
  // });
});
