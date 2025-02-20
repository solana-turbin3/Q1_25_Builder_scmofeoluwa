import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";
import { generateMints } from "../scripts/mint";
import { PublicKey } from "@solana/web3.js";
import { Yieldforge } from "../target/types/yieldforge";
import { assert } from "chai";

describe("yieldforge", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Yieldforge as Program<Yieldforge>;

  before(() => {});

  it("Initialize vault", async () => {
    const seed = new anchor.BN(1);

    const [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    const wallet = anchor.web3.Keypair.generate();

    // Airdrop some SOL to the wallet
    const connection = anchor.getProvider().connection;
    const airdropSignature = await connection.requestAirdrop(
      wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    const blockhash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature: airdropSignature,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });

    const usdcMint = await createMint(
      provider.connection,
      wallet,
      provider.wallet.publicKey,
      null,
      6
    );

    const collateralMint = await createMint(
      provider.connection,
      wallet,
      provider.wallet.publicKey,
      null,
      6
    );

    const tx = await program.methods
      .initialize(seed, provider.wallet.publicKey)
      .accounts({
        authority: provider.wallet.publicKey,
        usdcMint: usdcMint,
        collateralMint: collateralMint,
      })
      .rpc();

    console.log("Vault successfully initialized", tx);

    const vaultAccount = await program.account.vault.fetch(vaultPda);

    assert.ok(vaultAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(vaultAccount.seed.eq(seed));
  });
});
