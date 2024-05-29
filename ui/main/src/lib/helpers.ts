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


export function address_formater(addr: string){
    const front = addr.substring(0, 6);
    const back = addr.substring(addr.length - 6);

    return front + "..." + back;
}

export async function get_user_nft_list(userAddress: string){
	const rpcEndpoint = config.rpc;
	const nftContractAddress = config.nft_contract_address;
	const client = await CosmWasmClient.connect(rpcEndpoint);
	const queryMsg = {
		tokens: {
		  owner: userAddress,
		}
	};

	try {
	  const result = client.queryContractSmart(nftContractAddress, queryMsg);
	  return result;
	} catch (error) {
	  console.error("Error querying NFTs owned by user:", error);
	  return null;
	}
}

export async function get_contract_nft_list(){
	const rpcEndpoint = config.rpc;
	const nftContractAddress = config.nft_contract_address;
	const owner = config.swap_contract_address;
	const client = await CosmWasmClient.connect(rpcEndpoint);
	const queryMsg = {
		tokens: {
		  owner: owner,
		}
	};
	try {
	  const result = client.queryContractSmart(nftContractAddress, queryMsg);
	  return result;
	} catch (error) {
	  console.error("Error querying NFTs owned by user:", error);
	  return null;
	}
}

export async function get_balance() {
	const rpcEndpoint = config.rpc;
	const swapContractAddress = config.swap_contract_address;
	const client = await CosmWasmClient.connect(rpcEndpoint);
	const queryMsg = {
		get_balances: {}
	};
	try {
	  const result = client.queryContractSmart(swapContractAddress, queryMsg);
	  return result;
	} catch (error) {
	  console.error("Error querying balances:", error);
	  return null;
	}
}

export async function get_nft_balance() {
	const rpcEndpoint = config.rpc;
	const client = await CosmWasmClient.connect(rpcEndpoint);
    const owner_queryMsg = { tokens: { owner: config.swap_contract_address } };
    try {
        const result = await client.queryContractSmart(config.nft_contract_address, owner_queryMsg);
        return result;
    } catch (error) {
        console.error('Error querying NFT count:', error);
        return 0;
    }
}

export function convert_human_readable_balance(num: number): string {
    let strNum: string = num.toString();
    
    let commaIndex = strNum.length % 3;
    if (commaIndex === 0) {
        commaIndex = 3;
    }
    
    let result: string = strNum.slice(0, commaIndex);
    for (let i = commaIndex; i < strNum.length; i += 3) {
        result += ',' + strNum.slice(i, i + 3);
    }
    
    return result;
}

function getWalletSync(): any {
    let wallet;
    const unsubscribe = _wallet.subscribe(value => {
        wallet = value;
    });
    unsubscribe(); // 구독 즉시 해제
    return wallet;
}


// 토큰으로 nft 구매.
// nft 는 contract 소유.
export async function swap_token_to_nft(token_id: string) {
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	const instruction: any = {
		contractAddress: config.swap_contract_address,
		msg: {
			create_transaction: {
				desired_item: "nft",
				nft_token_id: token_id,
			}
		}
	}

	instruction.funds = [{
		denom: config.token_denom_address,
		amount: config.exchange_rate
	}]

	let instructions = []
	instructions.push(instruction)
	try{
		const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
		console.log(mintReceipt);
	}catch(e: any){
		console.log(e);
	}
}

async function get_approve(token_id: string){
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	const instruction: any = {
		contractAddress: config.nft_contract_address,
		msg: {
			approve: {
				spender: config.swap_contract_address,
				token_id: token_id,
			  }
		}
	}

	let instructions = []
	instructions.push(instruction)
	try{
		// const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
		// console.log(mintReceipt);
	}catch(e: any){
		console.log(e);
	}
}

export async function swap_nft_to_token(token_id: string) {
	let wallet = getWalletSync();
	const client = await getSigningCosmWasmClient(config.rpc, wallet.offlineSigner, {
		gasPrice: GasPrice.fromString("0.01usei")
	});

	// nft contract 에게 approve 신청 해야함.
	// 아니면 operator 로 등록해두기.

	get_approve(token_id).then(async ()=>{
		const instruction: any = {
			contractAddress: config.swap_contract_address,
			msg: {
				create_transaction: {
					desired_item: "token",
					nft_token_id: token_id,
				}
			}
		}
	
		let instructions = []
		instructions.push(instruction)
			const mintReceipt = await client.executeMultiple(wallet.accounts[0].address, instructions, "auto")
			console.log(mintReceipt);
	}).catch((err) => {
		console.log(err);
	})

}