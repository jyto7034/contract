import { connect, getSigningCosmWasmClient, type WalletConnect } from '@sei-js/core';
import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { GasPrice } from "@cosmjs/stargate";
import { _wallet } from "../lib/store"
import config from "../../config.json"

export const connectWallet = (name: any) => {
	return connect(name, config.chain)
		.then((wallet: WalletConnect) => {
			return wallet;
		})
		.catch((err: any) => {
			console.log(err);
		});
};

function getWalletSync(): any {
    let wallet;
    const unsubscribe = _wallet.subscribe(value => {
        wallet = value;
    });
    unsubscribe(); // 구독 즉시 해제
    return wallet;
}

export async function mint(_amount: String){
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	const instruction: any = {
		contractAddress: config.contract_address,
		msg: {
			mint_token: {
				amount: _amount
			}
		}
	}

	let instructions = []
	instructions.push(instruction)
	try{
		const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
		console.log(mintReceipt);
	}catch(e: any){
		console.log(e);
	}
}

export async function burn(_amount: String){
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	const instruction: any = {
		contractAddress: config.contract_address,
		msg: {
			burn_token: {
				amount: _amount
			}
		}
	}

	let instructions = []
	instructions.push(instruction)
	try{
		const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
		console.log(mintReceipt);
	}catch(e: any){
		console.log(e);
	}
}

export async function send(recipient: String, _amount: String, flag: boolean){
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	const instruction: any = {
		contractAddress: config.contract_address,
		msg: {
			send_token: {
				recipient: recipient,
				amount: _amount
			}
		}
	}

	if (flag){
		instruction.funds = [{
			denom: "factory/" + config.contract_address + "/chickegg",
			amount: _amount
		}]
	}

	let instructions = []
	instructions.push(instruction)
	try{
		const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
		console.log(mintReceipt);
	}catch(e: any){
		console.log(e);
	}
}

export async function get_balance() {
	const rpcEndpoint = config.rpc;
	const swapContractAddress = config.contract_address;
	const client = await CosmWasmClient.connect(rpcEndpoint);
	const queryMsg = {
		get_balances: {}
	};
	try {
	  const result = client.queryContractSmart(swapContractAddress, queryMsg);
	  return result;
	} catch (error) {
	  console.error("Error querying NFTs owned by user:", error);
	  return null;
	}
}