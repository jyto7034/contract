<script lang="ts">
	import Send from '../components/send.svelte';
	import Burn from '../components/burn.svelte';
	import Mint from '../components/mint.svelte';

	import { _wallet } from '../lib/store.ts'
	import { connectWallet } from '../lib/helpers.ts'

	let img = 'public/egg.PNG';
	let redirect = 'public/favicon.png';
	let discord_re = 'public/discord.PNG'
	let x_re = 'public/x.PNG'

	let chick_img = ['public/left_chick.PNG', 'public/mid_chick.PNG', 'public/right_chick.PNG']

	let tabs = ['Send', 'Mint', 'Burn'];
	let activeTab = 'Send';

	function selectTab(tab) {
		activeTab = tab;
	}

	async function connect_wallet() {
		connectWallet('compass')
			.then((wallet) => {
				_wallet.set(wallet);
			})
			.catch((err) => {
				console.log(err);
			});
	}
	connect_wallet();
</script>

<div class="container flex flex-col h-screen max-w-full">
	<div class="head flex flex-row h-[100px] justify-between">
		<div class="logo flex flex-row">
			<img src={img} clas alt="img" s="w-26 h-26 mt-2" />
			<p class="text-2xl flex items-center mb-2">chickegg</p>
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
		</div>
	</div>

	<div class="body h-[80vh] flex justify-center">
		{#if activeTab == 'Send'}
			<Send/>
		{:else if activeTab == 'Mint'}
			<Mint/>
		{:else if activeTab == 'Burn'}
			<Burn/>
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
