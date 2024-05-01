<script lang="ts">
	import Listbox from './listbox.svelte';
	import { connectWallet, address_formater, get_user_nft_list, get_balance, convert_human_readable_balance, get_contract_nft_list, get_nft_balance} from '../lib/helpers.ts';
	import { createEventDispatcher } from 'svelte';
	import { bridge_descriptions } from '../lib/descriptions.ts'
	import { _is_connect, _address, _data_cached, _nft_list, _token_list, _wallet  } from '../lib/store.ts'
	import config from "../../config.json"

	
	let is_connect = false;
	_is_connect.subscribe((value) => {
		is_connect = value;
	});

	let address = '';
	_address.subscribe((value) => {
		address = value;
	});

	let data_cached = false;
	_data_cached.subscribe((value) => {
		data_cached = value;
	});

	let nft_list = [];
	_nft_list.subscribe((value) => {
		nft_list = value;
	});

	let token_list = [];
	_token_list.subscribe((value) => {
		token_list = value;
	});

	const dispatch = createEventDispatcher();

    let isLoadingNFTs = true;
    let isLoadingTokens = true;
	let token_balance = 'Loading...'
	let nft_balance = 'Loading...'

	export function connect_wallet() {
		connectWallet('compass')
			.then((wallet) => {
				_wallet.set(wallet);
				let addr = wallet.accounts[0].address;

				_address.update( data => data = addr);
				_is_connect.update(n => n = true);
				get_nft_list();
				get_token_list();
				dispatch('message', { connected: addr });
			})
			.catch((err) => {
				console.log(err);
			});
	}

    function get_nft_list() {
        isLoadingNFTs = true;
        get_user_nft_list(address).then((data) =>{
            nft_list = data.tokens;
			_nft_list.update(n => n = nft_list);
            isLoadingNFTs = false;
			_data_cached.update(n => n = true);
        }).catch((err)=>{
			console.log(err);
            isLoadingNFTs = false;
        })
    }

    function get_token_list() {
		isLoadingTokens = true;
        get_contract_nft_list().then((data) => {
			token_list = data.tokens;
			_token_list.update(n => n = token_list);
            isLoadingTokens = false;
			_data_cached.update(n => n = true);
        }).catch((err) => {
			console.log(err);
            isLoadingTokens = false;
        })
    }

	if (is_connect){
		if (!data_cached){
			get_nft_list();
			get_token_list();
		}
	}

	if (data_cached){
		isLoadingNFTs = false;
    	isLoadingTokens = false;
	}

	
	function _get_token_balance(){
		get_balance().then(result => {
		const foundCoin = result.coin.find(c => c.denom == config.denom);
        token_balance = convert_human_readable_balance(foundCoin.amount);
		}).catch(error => {
			token_balance = '0';
			console.error(error);
		});
	}
	_get_token_balance();

	function _get_nft_balance(){
		get_nft_balance().then(result => {
			if (result.tokens.length > 0){
				nft_balance = result.tokens.length;
		}else{
			nft_balance = 0;
		}
		}).catch(error => {
			nft_balance = 'Error fetching balance';
			console.error(error);
		});
	}
	_get_nft_balance();

	function refresh_nft(event){
		refresh_nft();
		_get_nft_balance();
	}

	function refresh_token(event){
		get_token_list();
		_get_token_balance();
	}
</script>

<div class="body h-[800px] w-[1700px] flex justify-center">
	<div class="content-container h-[600px] w-[500px] mt-28 mr-20 outline outline-4 outline-offset-2">
		<div class="address m-6 ml-12">
			{#each bridge_descriptions as str}
				<p class="mt-6 text-2xl">{@html str}</p>
			{/each}
			
			<p class="mt-6 text-2xl">{@html `$EGG Escrow Token Balance: <br>`}</p>
			<p class="text-2xl">{token_balance}</p>
			<p class="text-2xl">{@html `Chicken NFT Balance: <br>`}</p>
			<p class="text-2xl">{nft_balance}</p>

		</div>
	</div>
	<div
		class="content-container h-[600px] w-[800px] mt-28 mr-[300px] outline outline-4 outline-offset-2"
	>
		{#if is_connect}
			<div class="flex flex-row h-[500px]">
				<div class="swap-nft m-10 ml-20 w-[280px]">
					<div class="sector-name text-3xl">
						<p>Swap NFT</p>
					</div>
					<div class="desc mt-10 text-lg">
						<p>You need 1 NFT in your wallet to</p>
						<p>swap to $EGG Tokens. Go and</p>
						<div class="flex flex-row">
							<p>buy on</p>
							<div class="flex flex-col">
								<a class="ml-3" href="https://pallet.exchange/collection/chickegg-v2" target="_blank">
									Pallet
								</a>
								<div class="px-4 ml-3 bg-white py-[1px]"></div>
							</div>
						</div>
					</div>
				    {#if isLoadingNFTs}
						<p class="mt-14 text-3xl">Loading...</p>
					{/if}
					{#if !isLoadingNFTs}
						<Listbox data_list={nft_list} button_name={"$EGG"} on:message={refresh_nft}/>
					{/if}
				</div>

				<div class="swap-token m-10 ml-20 w-[280px]">
					<div class="sector-name text-3xl">
						<p>Swap Token</p>
					</div>
					<div class="desc mt-10 text-lg">
						<p>You need 800,000 $EGG</p>
						<p>HYBRID tokens to swap</p>
						<p>for 1 chickegg NFT</p>
					</div>
				    {#if isLoadingTokens}
						<p class="mt-14 text-3xl">Loading...</p>
					{/if}
					{#if !isLoadingTokens}
						<Listbox data_list={token_list} button_name={"NFT"} on:message={refresh_token}/>
					{/if}
				</div>
			</div>
		{:else}
			<div class="m-14 flex flex-col items-center">
				<span class="text-2xl">You must connect your wallet to have access</span>
				<span class="text-2xl">the chickegg bridge</span>
			</div>

			<div class="w-full flex justify-center">
				<button
					class="px-9 py-3 text-3xl outline outline-1 outline-offset-1 transition duration-200 hover:bg-gray-400" on:click = { () => connect_wallet() }
				>
					connect
				</button>
			</div>
		{/if}
	</div>
</div>
