const { connect, KeyPair, keyStores, utils, Contract } = require("near-api-js");

const BN = require("bn.js");
const fs = require("fs").promises;
const assert = require("assert").strict;
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID = "ewtd.testnet";
const CONTRACT_ID = "nft.nephilim.testnet";
const WASM_PATH = "./contracts/main.wasm";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
const config = {
	keyStore,
	networkId: "testnet",
	nodeUrl: "https://rpc.testnet.near.org",
	headers: {},
};

main();

async function main() {
	const contract = await initContract(CONTRACT_ID);
	switch (process.argv[2]) {
		case "deploy":
			deployContract(CONTRACT_ID, WASM_PATH);
			break;
		case "get-tokens":
			const metadata = await contract.nft_tokens({});
			console.log(metadata);
			break;
		default:
	}
}

async function initContract(contractId) {
	const near = await connect(config);
	const account = await near.account(contractId);
	const methodOptions = {
		viewMethods: ["nft_metadata", "nft_tokens"],
		changeMethods: ["addMessage"],
	};
	return new Contract(account, contractId, methodOptions);
}

async function deployContract(contractId, wasmPath) {
	const near = await connect(config);
	const account = await near.account(contractId);
	const file = await fs.readFile(wasmPath);
	const result = await account.deployContract(file);
	console.log(result);
}
