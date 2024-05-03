<script lang="ts">
	import Home from '../components/home.svelte';
	import Bridge from '../components/bridge.svelte';
	import Stats from '../components/stats.svelte';
	import Swap from '../components/swap.svelte';

	import { _is_connect, _address, _wallet } from '../lib/store.ts'
	import { connectWallet, address_formater, get_user_nft_list } from '../lib/helpers.ts';

	let is_connect = false;
	_is_connect.subscribe((value) => {
		is_connect = value;
	});

	let address = '';
	_address.subscribe((value) => {
		address = value;
	});
	let img = 'public/egg.PNG';
	let redirect = 'public/favicon.png';
	let discord_re = 'public/discord.PNG'
	let x_re = 'public/x.PNG'

	let chick_img = ['public/left_chick.PNG', 'public/mid_chick.PNG', 'public/right_chick.PNG']

	let bridgeComponent;
	let tabs = ['Bridge', 'Stats', 'Swap'];
	let activeTab = 'Home';
	let edited_addr = '';

	function selectTab(tab) {
		if (tab === 'Stats') {
			// 'Stats' 탭을 클릭해도 아무 반응이 없도록 설정
			return;
		}
		activeTab = tab;
	}

	async function connect_wallet() {
		connectWallet('compass')
			.then((wallet) => {
				_wallet.set(wallet);
				let addr = wallet.accounts[0].address;
				
				_address.update( data => data = addr);
				_is_connect.update(n => n = true);
				edited_addr = address_formater(addr);

				bridgeComponent.connect_wallet();
			})
			.catch((err) => {
				console.log(err);
			});
	}

	function handleMessage(event) {
		_is_connect.update(n => n = true);
		edited_addr = address_formater(event.detail.connected);
	}
</script>

<div class="container flex flex-col h-screen max-w-full">
	<div class="head flex flex-row h-[100px] justify-between">
		<div class="logo flex flex-row">
			<img src={img} clas alt="img" s="w-26 h-26 mt-2" />
			<button on:click={() => selectTab('Home')}>
				<p class="text-2xl flex items-center mb-2">chickegg</p>
			</button>
		</div>

		<div class="tabs static h-full w-2/5 flex flex-row justify-between">
			{#each tabs as tab}
				<div class="h-full flex flex-col items-center">
					{#if tab == activeTab}
						<button
							class="h-full text-white text-2xl font-bold px-10 button-effect"
							on:click={() => selectTab(tab)}
						>
							{tab}
						</button>
					{:else}
						<button
							class="h-full text-gray-700 text-2xl font-bold px-10 button-effect"
							on:click={() => selectTab(tab)}
						>
							{tab}
						</button>
					{/if}
				</div>
			{/each}
		</div>

		<div class="redirect flex flex-row items-center">
			<div class="px-3 py-3 button-effect hover:cursor-pointer">
				<a href="https://twitter.com/chickeggHYBRID" target="_blank">
					<img src={x_re} alt="img" class="w-16 h-16" />
				</a>
			</div>
			<div class="px-3 py-3 button-effect hover:cursor-pointer">
				<a href="https://example.com" target="_blank">
					<img src={discord_re} alt="img" class="w-16 h-16 pt-1 pb-1" />
				</a>
			</div>
		</div>

		<div class="wallet h-full w-32 flex flex-row justify-end">
			{#if is_connect == false}
				<button
					class="text-white text-2xl font-bold px-8 button-effect" on:click={() => connect_wallet()}>
					Connect
				</button>
			{:else}
			<!-- TODO : Click to Copy -->
				<button class="text-white text-2xl font-bold px-8 button-effect"> {edited_addr} </button>
			{/if}
		</div>
	</div>

	<div class="body h-[80vh] flex justify-center">
		{#if activeTab == 'Home'}
			<Home/>
		{:else if activeTab == 'Bridge'}
			<Bridge on:message={handleMessage} bind:this={bridgeComponent}/>
		{:else if activeTab == 'Stats'}
			<Stats/>
		{:else if activeTab == 'Swap'}
			<Swap/>
		{/if}
	</div>

	<div class="footer h-[10vh] flex justify-between items-center ml-9 mr-9 mb-4">
		<img src={chick_img[0]} alt="img" class="w-32 h-32" />
		<img src={chick_img[1]} alt="img" class="w-32 h-26" />
		<img src={chick_img[2]} alt="img" class="w-32 h-26" />
	</div>
</div>

<style lang="postcss">
	:global(html) {
		background-color: black;
		color: white;
	}
	:global(body) {
		margin: 0;
	}
	@media (max-height: 900px) {
		.footer {
			display: none;
		}
	}
</style>
