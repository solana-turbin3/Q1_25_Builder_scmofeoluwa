import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Yieldforge } from "../target/types/yieldforge";
import { assert, expect } from "chai";
import { fundUsdc } from "../scripts/bankrun";
import walletPair from "./dev-wallet.json";
import userPair from "./wallet.json";

describe("yieldforge", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Yieldforge as Program<Yieldforge>;

  const solendProgram = new PublicKey(
    "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"
  );
  const kaminoProgram = new PublicKey(
    "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
  );

  let connection: anchor.web3.Connection;
  let depositAmount: number;
  let vaultPda: PublicKey;
  let wallet: Keypair;
  let user: Keypair;
  let userUsdcAta: PublicKey;
  let userState: PublicKey;
  let usdcMint: PublicKey;
  let collateralMint: PublicKey;

  let reserve: PublicKey;
  let liquiditySupply: PublicKey;
  let lendingMarket: PublicKey;
  let lendingMarketAuthority: PublicKey;

  before(async () => {
    connection = provider.connection;

    // Import keypair from the wallet file
    // wallet = Keypair.fromSecretKey(new Uint8Array(walletPair));
    // user = Keypair.fromSecretKey(new Uint8Array(userPair));
    wallet = anchor.web3.Keypair.generate();
    user = anchor.web3.Keypair.generate();

    // Airdrop SOL
    const walletAirdropSig = await connection.requestAirdrop(
      wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    const userAirdropSig = await connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    const blockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: walletAirdropSig,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });
    await connection.confirmTransaction({
      signature: userAirdropSig,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    });

    // Find PDAs
    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer()],
      program.programId
    );

    [userState] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), wallet.publicKey.toBuffer()],
      program.programId
    );

    usdcMint = await createMint(connection, wallet, wallet.publicKey, null, 6);
    depositAmount = 1000000;

    collateralMint = await createMint(
      connection,
      wallet,
      wallet.publicKey,
      null,
      6
    );

    const userUsdcAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      usdcMint,
      user.publicKey
    );
    userUsdcAta = userUsdcAccount.address

    // Fund user with test usdc
    await mintTo(
      connection,
      wallet,
      usdcMint,
      userUsdcAta,
      wallet.publicKey,
      10_000_000
    );
  });

  it("Initialize vault", async () => {
    const seed = new anchor.BN(2);

    const tx = await program.methods
      .initialize(seed)
      .accounts({
        authority: wallet.publicKey,
        usdcMint: usdcMint,
        collateralMint: collateralMint,
      })
      .signers([wallet])
      .rpc();

    console.log("Vault successfully initialized", tx);

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    assert.ok(vaultAccount.authority.equals(wallet.publicKey));
    assert.ok(vaultAccount.seed.eq(seed));
  });

  // it("Deposit into Vault", async () => {
  //   // Initial user deposit
  //   const userAta = await connection.getTokenAccountBalance(userUsdcAta)
  //   const initialUserBalance = userAta.value.uiAmount
  //
  //   // Deposit into vault
  //   const tx = await program.methods
  //     .depositIntoVault(new anchor.BN(depositAmount))
  //     .accounts({
  //       user: user.publicKey,
  //       usdcMint: usdcMint,
  //     })
  //     .signers([user])
  //     .rpc();
  //
  //   console.log("User successfully deposited into vault", tx);
  //
  //   // Verify the deposit
  //   const finalUserBalance = userAta.value.uiAmount
  //   expect(finalUserBalance - initialUserBalance).to.equals(depositAmount);
  // });
  //
  // it("Deposit into Solend", async () => {
  //   usdcMint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  //   collateralMint = new PublicKey(
  //     "993dVFL2uXWYeoXuEBFXR4BijeXdTv4s6BzsCjJZuwqk"
  //   );
  //   reserve = new PublicKey("BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw");
  //   liquiditySupply = new PublicKey(
  //     "8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf"
  //   );
  //   lendingMarket = new PublicKey(
  //     "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY"
  //   );
  //   lendingMarketAuthority = new PublicKey(
  //     "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby"
  //   );
  //
  //   // Fund vault with test USDC
  //   await fundUsdc(vaultPda, usdcMint.toBase58());
  //
  //   //Initial vault cUSDC balance
  //   const vaultAccount = await program.account.vault.fetch(vaultPda);
  //   const initialCollateralBalance = vaultAccount.totalCUsdc.toNumber();
  //
  //   const tx = await program.methods
  //     .depositIntoSolend(new anchor.BN(depositAmount))
  //     .accounts({
  //       reserve: reserve,
  //       reserveLiquiditySupply: liquiditySupply,
  //       reserveCollateralMint: collateralMint,
  //       lendingMarket: lendingMarket,
  //       lendingMarketAuthority: lendingMarketAuthority,
  //       solendProgram: solendProgram,
  //     })
  //     .rpc();
  //
  //   console.log("Successfully Deposited:", tx);
  //
  //   // Verify the deposit
  //   const finalCollateralBalance = vaultAccount.totalCUsdc.toNumber();
  //   expect(finalCollateralBalance - initialCollateralBalance).to.equals(
  //     depositAmount
  //   );
  // });
  //
  // it("Withdraw from Solend", async () => {
  //   //Initial vault cUSDC balance
  //   const vaultAccount = await program.account.vault.fetch(vaultPda);
  //   const initialCollateralBalance = vaultAccount.totalCUsdc.toNumber();
  //
  //   const tx = await program.methods
  //     .withdrawFromSolend(new anchor.BN(depositAmount))
  //     .accounts({
  //       reserve: reserve,
  //       reserveLiquiditySupply: liquiditySupply,
  //       reserveCollateralMint: collateralMint,
  //       lendingMarket: lendingMarket,
  //       lendingMarketAuthority: lendingMarketAuthority,
  //       solendProgram: solendProgram,
  //     })
  //     .rpc();
  //
  //   console.log("Successfully Deposited:", tx);
  //
  //   // Verify the withdrawal
  //   const finalCollateralBalance = vaultAccount.totalCUsdc.toNumber();
  //   expect(initialCollateralBalance - finalCollateralBalance).to.equals(
  //     depositAmount
  //   );
  // });
  //
  // it("Deposit into Kamino", async () => {
  //   usdcMint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  //   collateralMint = new PublicKey(
  //     "B8V6WVjPxW1UGwVDfxH2d2r8SyT4cqn7dQRK6XneVa7D"
  //   );
  //   const reserve = new PublicKey(
  //     "D6q6wuQSrifJKZYpR1M8R4YawnLDtDsMmWM1NbBmgJ59"
  //   );
  //   const liquiditySupply = new PublicKey(
  //     "Bgq7trRgVMeq33yt235zM2onQ4bRDBsY5EWiTetF4qw6"
  //   );
  //   const lendingMarket = new PublicKey(
  //     "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
  //   );
  //   const lendingMarketAuthority = new PublicKey(
  //     "9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo"
  //   );
  //   const kaminoProgram = new PublicKey(
  //     "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
  //   );
  //
  //   // Fund vault with test USDC
  //   await fundUsdc(vaultPda, usdcMint.toBase58());
  //
  //   //Initial vault kUSDC balance
  //   const vaultAccount = await program.account.vault.fetch(vaultPda);
  //   const initialCollateralBalance = vaultAccount.totalKUsdc.toNumber();
  //
  //   const tx = await program.methods
  //     .depositIntoKamino(new anchor.BN(depositAmount))
  //     .accounts({
  //       reserve: reserve,
  //       reserveLiquidityMint: usdcMint,
  //       reserveLiquiditySupply: liquiditySupply,
  //       reserveCollateralMint: collateralMint,
  //       lendingMarket: lendingMarket,
  //       lendingMarketAuthority: lendingMarketAuthority,
  //       kaminoProgram: kaminoProgram,
  //       sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
  //     })
  //     .rpc();
  //
  //   console.log("Successfully Deposited:", tx);
  //
  //   // Verify the deposit
  //   const finalCollateralBalance = vaultAccount.totalKUsdc.toNumber();
  //   expect(finalCollateralBalance - initialCollateralBalance).to.equals(
  //     depositAmount
  //   );
  // });
  //
  // it("Withdraw from Kamino", async () => {
  //   //Initial vault kUSDC balance
  //   const vaultAccount = await program.account.vault.fetch(vaultPda);
  //   const initialCollateralBalance = vaultAccount.totalKUsdc.toNumber();
  //
  //   const tx = await program.methods
  //     .withdrawFromKamino(new anchor.BN(depositAmount))
  //     .accounts({
  //       reserve: reserve,
  //       reserveLiquidityMint: usdcMint,
  //       reserveLiquiditySupply: liquiditySupply,
  //       reserveCollateralMint: collateralMint,
  //       lendingMarket: lendingMarket,
  //       lendingMarketAuthority: lendingMarketAuthority,
  //       kaminoProgram: kaminoProgram,
  //       sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
  //     })
  //     .rpc();
  //
  //   console.log("Successfully Deposited:", tx);
  //
  //   // Verify the withdrawal
  //   const finalCollateralBalance = vaultAccount.totalKUsdc.toNumber();
  //   expect(initialCollateralBalance - finalCollateralBalance).to.equals(
  //     depositAmount
  //   );
  // });
  //
  // it("Withdraw from Vault", async () => {
  //   // Initial user deposit
  //   const userAta = await connection.getTokenAccountBalance(userUsdcAta)
  //   const initialUserBalance = userAta.value.uiAmount
  //
  //   // Deposit into vault
  //   const tx = await program.methods
  //     .withdrawFromVault(new anchor.BN(depositAmount))
  //     .accounts({
  //       user: user.publicKey,
  //       usdcMint: usdcMint,
  //     })
  //     .signers([user])
  //     .rpc();
  //
  //   console.log("User successfully deposited into vault", tx);
  //
  //   // Verify the withdrawal
  //   const finalUserBalance = userAta.value.uiAmount
  //   expect(initialUserBalance - finalUserBalance).to.equals(depositAmount);
  // });
});
