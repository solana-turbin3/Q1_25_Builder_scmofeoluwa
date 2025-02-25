import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "../target/types/smartwallet";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";

describe("smartwallet", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Smartwallet as Program<Smartwallet>;
  const seed = new anchor.BN(1);
  let keypair: Keypair;
  let walletPda: PublicKey;
  let connection: anchor.web3.Connection;

  before("setup test", async () => {
    keypair = anchor.web3.Keypair.generate();
    connection = anchor.getProvider().connection;

    // Airdrop SOL
    const airdropSig = await connection.requestAirdrop(
      keypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    const blockhash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature: airdropSig,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });

    [walletPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("wallet"), keypair.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initialize vault", async () => {
    const tx = await program.methods
      .initialize(keypair.publicKey, seed)
      .accounts({
        owner: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
    console.log("Your transaction signature", tx);

    const walletAccount = await program.account.wallet.fetch(walletPda);
    assert.ok(walletAccount.owner.equals(keypair.publicKey));
    assert.ok(walletAccount.seed.eq(seed))
  });

  it("Withdraw SOL from vault", async () => {
    const airdropSig = await connection.requestAirdrop(
      walletPda,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    const blockhash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature: airdropSig,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });
    const initialWalletBalance = await connection.getBalance(walletPda)
    const initialOwnerBalance = await connection.getBalance(keypair.publicKey)

    // withdraw sol
    const tx = await program.methods.withdrawSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
      destination: keypair.publicKey,
      owner: keypair.publicKey,
    }).signers([keypair]).rpc()
    console.log("Your transaction signature", tx);

    const finalWalletBalance = await connection.getBalance(walletPda)
    const finalOwnerBalance = await connection.getBalance(keypair.publicKey)

    expect(initialWalletBalance - finalWalletBalance).to.equals(1 * anchor.web3.LAMPORTS_PER_SOL)
    expect(finalOwnerBalance - initialOwnerBalance).to.equals(1 * anchor.web3.LAMPORTS_PER_SOL)
  })
});
