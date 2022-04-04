const { connect, KeyPair, keyStores, utils, Contract } = require("near-api-js");

const BN = require("bn.js");
const fs = require("fs").promises;
const assert = require("assert").strict;
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID = "nephilim.testnet";
const CONTRACT_ID = "nft4.nephilim.testnet";
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
		case "init":
			await contract.new_default_meta({ args: { owner_id: ACCOUNT_ID } });
			break;
		case "migrate":
			await contract.migrate({ args: { owner_id: ACCOUNT_ID } });
			break;
		case "update-uri":
			await contract.update_uri({
				args: { uri: "https://ewtd.mypinata.cloud/ipfs/QmTLoZs1v2WoYnG1UQF4h8j4wEfGwzALv2A8iw9zvTQU8m" },
			});
			break;

		case "metadata":
			const metadata = await contract.nft_metadata({});
			console.log(metadata);
			break;
		case "flip-public":
			await contract.flip_public_sale({ args: {} });
			break;
		case "flip-presale":
			await contract.flip_presale({ args: {} });
			break;
		case "mint":
			for (let i = 0; i < 1332; i++) {
				await contract.nft_mint({ args: {}, amount: utils.format.parseNearAmount("9.99"), gas: 300000000000000 });
			}

			break;
		case "status":
			const status = await contract.get_sale_status({ args: {} });
			console.log(status);
			break;
		case "tokens":
			const tokens = await contract.nft_tokens({ args: {} });
			console.log(tokens);
			break;
		case "whitelist-check":
			const isWhitelisted = await contract.is_whitelisted({ account_id: ACCOUNT_ID });
			console.log(isWhitelisted);
			break;
		case "whitelist-mint":
			await contract.whitelist_nft_mint({ args: {}, amount: utils.format.parseNearAmount("9.99") });
			break;
		case "whitelist-apply":
			await contract.apply_for_whitelist({ args: {} });
			break;
		case "get-applied":
			const applied = await contract.get_applied_id({ args: {} });
			console.log(applied);
			break;
		case "raffle-whitelist":
			const winner = await contract.raffle_whitelist({ args: {} });
			console.log(winner);
			break;
		case "raffle-free-mint":
			const winnerFree = await contract.raffle_free_mint({ args: {} });
			console.log(winnerFree);
			break;
		case "whitelist-add":
			await contract.add_to_whitelist({ args: { account_id: ACCOUNT_ID, amount: 1 } });
			break;
		default:
	}
}

async function initContract(contractId) {
	const near = await connect(config);
	const account = await near.account(ACCOUNT_ID);
	const methodOptions = {
		viewMethods: ["nft_metadata", "nft_tokens", "get_sale_status", "is_whitelisted", "get_applied_id"],
		changeMethods: [
			"new_default_meta",
			"migrate",
			"update_uri",
			"flip_public_sale",
			"flip_presale",
			"transfer_ownership",
			"get_owner",
			"whitelist_nft_mint",
			"nft_mint",
			"add_to_whitelist",
			"apply_for_whitelist",
			"raffle_whitelist",
			"raffle_free_mint",
			,
		],
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
