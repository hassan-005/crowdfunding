const anchor = require("@coral-xyz/anchor");
const assert = require("assert");
const { clusterApiUrl, LAMPORTS_PER_SOL } = require("@solana/web3.js")
const { SystemProgram } = anchor.web3;
const fs = require("fs");
const { it } = require("mocha");

describe("gofundme", () => {
  const userKeypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('./id.json'))));
  // console.log(userKeypair.publicKey)
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection(clusterApiUrl("devnet")),
    new anchor.Wallet(userKeypair)
    // anchor.AnchorProvider.defaultOptions()
  );

  anchor.setProvider(provider);
  console.log(provider)
  const program = anchor.workspace.Gofundme;
  const campaign = anchor.web3.Keypair.generate();

  // it("Is initialized!", async () => {
  //   // console.log(userKeypair.publicKey);



  //   const tx = await program.rpc.create("name", "desc", {
  //     accounts: {
  //       campaign: campaign.publicKey,
  //       authority: userKeypair.publicKey,
  //       systemProgram: SystemProgram.programId
  //     },
  //     signers: [campaign]
  //   })
  //   // const acc = await program.account.campaign.fetch(campaign.publicKey);
  //   console.log("Initialized Tx: ",tx);


  // });
  // it("DonatING!", async () => {
  //   const tx = await program.rpc.donate(new anchor.BN(2 * LAMPORTS_PER_SOL), {
  //     accounts: {
  //       campaign: campaign.publicKey,
  //       authority: userKeypair.publicKey,
  //       systemProgram: SystemProgram.programId
  //     },
  //     signers: []
  //   })
  //   // const acc = await program.account.campaign.fetch(campaign.publicKey);
  //   console.log("Donated Tx: ",tx);


  // });

  // it("Withdrawing", async () => {
  //   const tx = await program.rpc.withdraw(new anchor.BN(1 * LAMPORTS_PER_SOL), {
  //     accounts: {
  //       campaign:  campaign.publicKey,
  //       authority: userKeypair.publicKey
  //     },
  //     signers: []
  //   })
  //   console.log("Withdrawn Tx: ",tx)
  // })
});
