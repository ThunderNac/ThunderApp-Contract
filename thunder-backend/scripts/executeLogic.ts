import { RpcProvider, Account, Contract } from "starknet";
import fs from "fs";
import path from "path";
import * as dotenv from "dotenv";
dotenv.config();
//import air_drop from '../abis/air_drop.json';
import { ec, hash, type BigNumberish, type WeierstrassSignatureType,stark } from 'starknet';
import { TypedData } from "starknet";
import { constants } from "starknet";



async function main() {
  const provider = new RpcProvider({
    nodeUrl:process.env.RPC_URL!,
  });

  const privateKey = process.env.PRIVATE_KEY!;
  const accountAddress = process.env.ACCOUNT_ADDRESS!;
  // const contractAddress = process.env.CONTRACT_ADDRESS!;

  const account = new Account(provider, accountAddress, privateKey);
  //const contract = new Contract(air_drop.abi, contractAddress, account);


const starknetPublicKey = ec.starkCurve.getStarkKey(privateKey);
const fullPublicKey = stark.getFullPublicKey(privateKey);

const orderTypedData: TypedData = {
  domain: {
    name: "OrderVault",
    chainId: constants.StarknetChainId.SN_SEPOLIA,
    version: "1.0.0",
    revision: "1", // requis par SNIP-12
  },
  primaryType: "Order",
  types: {
    Order: [
      { name: "user", type: "felt" },
      { name: "tokenIn", type: "felt" },
      { name: "tokenOut", type: "felt" },
      { name: "amountInLow", type: "u128" },
      { name: "amountInHigh", type: "u128" },
      { name: "apy", type: "felt" },
      { name: "deadline", type: "felt" },
      { name: "nonce", type: "felt" },
    ],
    StarknetDomain: [
      { name: "name", type: "shortstring" },
      { name: "chainId", type: "shortstring" },
      { name: "version", type: "shortstring" },
    ],
  },
  message: {
    user: accountAddress,              
    tokenIn: "0x07f...def",
    tokenOut: "0x06d...456",
    amountInLow: "12345678901234567890", // u256 = low/high (séparés en u128)
    amountInHigh: "0",
    apy: "1000",                         // 10% en base 10000
    deadline: "1718128000",             // timestamp unix
    nonce: "2"
  }
};

const msgHash = await account.hashMessage(orderTypedData);
const signature = await account.signMessage(orderTypedData);

//const msgHash = hash.computeHashOnElements(message);
//const signature: WeierstrassSignatureType = ec.starkCurve.sign(msgHash, privateKey);

  console.log("is everything working ? ✅",signature);


//   // Exemple : tokenIds [0,1,2] avec supply [150, 100, 50]
//   const tokenIds = [0, 1, 2];
//   const amounts = [150, 100, 50];

  // const tx = await contract.mintBatchForAirdrop(tokenIds, amounts);
  // console.log("Transaction sent:", tx.transaction_hash);

  // await provider.waitForTransaction(tx.transaction_hash);
  // console.log("✅ Mint batch complete!");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
