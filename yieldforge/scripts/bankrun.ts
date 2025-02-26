import { start } from "solana-bankrun";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  AccountLayout,
  TOKEN_PROGRAM_ID,
  ACCOUNT_SIZE,
} from "@solana/spl-token";

export async function fundUsdc(owner: PublicKey, mint: string) {
  const usdcMint = new PublicKey(mint);
  const ata = await getAssociatedTokenAddress(usdcMint, owner, true);
  const usdcToOwn = 1_000_000_000n;
  const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
  AccountLayout.encode(
    {
      mint: usdcMint,
      owner,
      amount: usdcToOwn,
      delegateOption: 0,
      delegate: PublicKey.default,
      delegatedAmount: 0n,
      state: 1,
      isNativeOption: 0,
      isNative: 0n,
      closeAuthorityOption: 0,
      closeAuthority: PublicKey.default,
    },
    tokenAccData
  );
  const context = await start(
    [],
    [
      {
        address: ata,
        info: {
          lamports: 1 * LAMPORTS_PER_SOL,
          data: tokenAccData,
          owner: TOKEN_PROGRAM_ID,
          executable: false,
        },
      },
    ]
  );
  const client = context.banksClient;
  const rawAccount = await client.getAccount(ata);
  const rawAccountData = rawAccount?.data;
  AccountLayout.decode(rawAccountData);

  return ata.toBase58();
}
