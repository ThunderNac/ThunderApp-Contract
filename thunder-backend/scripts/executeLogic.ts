import { RpcProvider, Account, Contract } from "starknet";
import fs from "fs";
import path from "path";
import * as dotenv from "dotenv";
dotenv.config();
//import air_drop from '../abis/air_drop.json';

type AirdropEntry = {
    addr: string;
    nbrHold: number;
  };
  
  async function loadJson(file: string): Promise<AirdropEntry[]> {
    const data = fs.readFileSync(path.join(__dirname, "../data", file), "utf-8");
    return JSON.parse(data);
  }
  
async function main() {
  const provider = new RpcProvider({
    nodeUrl:process.env.RPC_URL!,
  });

  const privateKey = process.env.PRIVATE_KEY!;
  const accountAddress = process.env.ACCOUNT_ADDRESS!;
  // const contractAddress = process.env.CONTRACT_ADDRESS!;

  const account = new Account(provider, accountAddress, privateKey);
  //const contract = new Contract(air_drop.abi, contractAddress, account);

  console.log("is everything working ? ✅",account);


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
