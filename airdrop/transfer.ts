import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// Import keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// Define Turbin3 public key
const to = new PublicKey("CgBgYuveQN4zmxYKSMu32yzzkWp1jtqu8oUhgmxvDvRC");

// Create a Solana devnet connection
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    // Get balance of dev wallet
    const balance = await connection.getBalance(from.publicKey);

    // Create a test transaction to calculate fees
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );

    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = from.publicKey;

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;

    // Remove the transfer instruction to replace it
    transaction.instructions.pop();

    // Add the instruction back with the correct amount of lamports
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    // Sign and send the transaction
    const signature = await sendAndConfirmTransaction(connection, transaction, [from]);
    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
