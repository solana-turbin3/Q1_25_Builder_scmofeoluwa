import wallet from "../../../airdrop/Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("6qBkvrg4hmEtKDWiRhenJKkdh3wumYN78az7eUt5hnUf")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
      // Start here
      let accounts: CreateMetadataAccountV3InstructionAccounts = {
        mint: mint,
        mintAuthority: signer,
      }

      let data: DataV2Args = {
        name: "MFer",
        symbol: "MFer",
        uri: "https://mfer.com",
        sellerFeeBasisPoints: 1,
        creators: null,
        collection: null,
        uses: null
      }

      let args: CreateMetadataAccountV3InstructionArgs = {
        data: data,
        isMutable: false,
        collectionDetails: null
      }

      let tx = createMetadataAccountV3(
        umi,
        {
        ...accounts,
        ...args
        }
      )

      let result = await tx.sendAndConfirm(umi);
      console.log(bs58.encode(result.signature));
    } catch(e) {
      console.error(`Oops, something went wrong: ${e}`)
    }
})();
