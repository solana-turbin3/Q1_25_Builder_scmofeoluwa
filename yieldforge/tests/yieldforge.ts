import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { PublicKey, Keypair } from "@solana/web3.js";
import { Yieldforge } from "../target/types/yieldforge";
import { assert } from "chai";
import keypair from "./wallet.json";

describe("yieldforge", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Yieldforge as Program<Yieldforge>;

  // Test accounts
  let usdcMint: PublicKey;
  let collateralMint: PublicKey;
  let vaultPda: PublicKey;
  let vaultBump: number;
  let wallet: Keypair;
  let mintAuthority: Keypair;
  let userUsdcAta: PublicKey;
  let userState: PublicKey;

  const KAMINO_PROGRAM_ID = new PublicKey(
    "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
  );
  const USDC_MINT = new PublicKey(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  );

  const KAMINO_MARKET = new PublicKey(
    "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
  );
  const USDC_RESERVE = new PublicKey(
    "9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo"
  );

  const LENDING_MARKET_AUTHORITY = PublicKey.findProgramAddressSync(
    [KAMINO_MARKET.toBuffer()],
    KAMINO_PROGRAM_ID
  )[0];

  before(async () => {
    // Create test user

    // Import keypair from the wallet file
    // wallet = Keypair.fromSecretKey(new Uint8Array(keypair));
    wallet = anchor.web3.Keypair.generate();
    mintAuthority = anchor.web3.Keypair.generate();
    const connection = anchor.getProvider().connection;

    // Airdrop SOL
    const airdropSig = await connection.requestAirdrop(
      wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    const blockhash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature: airdropSig,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });

    // Find PDAs
    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    [userState] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), wallet.publicKey.toBuffer()],
      program.programId
    );

    // usdcMint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    // collateralMint = new PublicKey("B8V6WVjPxW1UGwVDfxH2d2r8SyT4cqn7dQRK6XneVa7D")
    // Use actual USDC mint
    usdcMint = await createMint(
      provider.connection,
      wallet,
      wallet.publicKey,
      null,
      6
    );

    collateralMint = await createMint(
      provider.connection,
      wallet,
      wallet.publicKey,
      null,
      6
    );
  });

  it("Initialize vault", async () => {
    const seed = new anchor.BN(1);

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

  it("Deposit into Vault", async () => {
    // Create ATA for user
    const userUsdcAtaAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet,
      usdcMint,
      wallet.publicKey
    );
    userUsdcAta = userUsdcAtaAccount.address;

    // Fund user with test usdc
    await mintTo(
      provider.connection,
      wallet,
      usdcMint,
      userUsdcAta,
      wallet.publicKey,
      10_000_000 // 10 USDC in base unitt
    );

    // Deposit into vault
    const tx = await program.methods
      .depositIntoVault(new anchor.BN(1000000))
      .accounts({
        user: wallet.publicKey,
        usdcMint: usdcMint,
      })
      .signers([wallet])
      .rpc();

    console.log("USDC successfully deposited", tx);
  });

  it("Deposit into Solend", async () => {
    const reserveLiquiditySupplyAccount =
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        usdcMint,
        new PublicKey("8K1JqB3xfTTCp49MFeZe8FsdC5bbJpAThVjQgt7iHoLh")
      );
    const reserve = new PublicKey(
      "8K1JqB3xfTTCp49MFeZe8FsdC5bbJpAThVjQgt7iHoLh"
    );
    const depositAmount = 1000000;

    const tx = await program.methods
      .depositIntoKamino(new anchor.BN(1000000))
      .accounts({
        reserve: USDC_RESERVE,
        reserveLiquiditySupply: new PublicKey(
          "Bgq7trRgVMeq33yt235zM2onQ4bRDBsY5EWiTetF4qw6"
        ),
        reserveCollateralMint: new PublicKey(
          "B8V6WVjPxW1UGwVDfxH2d2r8SyT4cqn7dQRK6XneVa7D"
        ),
        reserveLiquidityMint: USDC_MINT,
        lendingMarket: new PublicKey(
          "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
        ),
        lendingMarketAuthority: LENDING_MARKET_AUTHORITY,
        kaminoProgram: new PublicKey(
          "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
        ),
        sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .rpc();

    // Verify the deposit
    const vaultAccount = await program.account.vault.fetch(vaultPda);
    const userStateAccount = await program.account.user.fetch(userState);

    assert.ok(vaultAccount.totalUsdcDeposits.eq(new anchor.BN(depositAmount)));
    assert.ok(userStateAccount.usdcDeposited.eq(new anchor.BN(depositAmount)));
  });
});
