import fs from "fs";
import path from "path";
import * as dotenv from "dotenv";
dotenv.config();
//import air_drop from '../abis/air_drop.json';
import { TypedData } from "starknet";
import { constants } from "starknet";
import {
    Account,
    ec,
    json,
    stark,
    RpcProvider,
    hash,
    CallData,
    CairoOption,
    CairoOptionVariant,
    CairoCustomEnum,
  } from 'starknet';



async function main() {
  const provider = new RpcProvider({
    nodeUrl:process.env.RPC_URL!,
  });

  const privateKey = process.env.PRIVATE_KEY!;
  const accountAddress = process.env.ACCOUNT_ADDRESS!;
  // const contractAddress = process.env.CONTRACT_ADDRESS!;

  const account = new Account(provider, accountAddress, privateKey);
  //const contract = new Contract(air_drop.abi, contractAddress, account);


//new Argent X account v0.4.0
const argentXaccountClassHash =
  '0x036078334509b514626504edc9fb252328d1a240e4e948bef8d0c08dff45927f';

// Generate public and private key pair.
const privateKeyAX = stark.randomAddress();
console.log('AX_ACCOUNT_PRIVATE_KEY=', privateKeyAX);
const starkKeyPubAX = ec.starkCurve.getStarkKey(privateKeyAX);
console.log('AX_ACCOUNT_PUBLIC_KEY=', starkKeyPubAX);

// Calculate future address of the ArgentX account
const axSigner = new CairoCustomEnum({ Starknet: { pubkey: starkKeyPubAX } });
const axGuardian = new CairoOption<unknown>(CairoOptionVariant.None);
const AXConstructorCallData = CallData.compile({
  owner: axSigner,
  guardian: axGuardian,
});
const AXcontractAddress = hash.calculateContractAddressFromHash(
  starkKeyPubAX,
  argentXaccountClassHash,
  AXConstructorCallData,
  0
);
console.log('Precalculated account address=', AXcontractAddress);


  const accountAX = new Account(provider, AXcontractAddress, privateKeyAX);

const deployAccountPayload = {
  classHash: argentXaccountClassHash,
  constructorCalldata: AXConstructorCallData,
  contractAddress: AXcontractAddress,
  addressSalt: starkKeyPubAX,
};

const { transaction_hash: AXdAth, contract_address: AXcontractFinalAddress } =
  await accountAX.deployAccount(deployAccountPayload);
console.log('✅ ArgentX wallet deployed at:', AXcontractFinalAddress);

console.log("is everything working ? ✅");



}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
