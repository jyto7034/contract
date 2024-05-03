<script lang="ts">
	import Home from '../components/home.svelte';
	import Bridge from '../components/bridge.svelte';
	import Stats from '../components/stats.svelte';
	import Swap from '../components/swap.svelte';

	import { _wallet } from '../lib/store.ts'
	import { connectWallet, address_formater, get_user_nft_list } from '../lib/helpers.ts';


	async function connect_wallet() {
		connectWallet('compass')
			.then((wallet) => {
				_wallet.set(wallet);
				let addr = wallet.accounts[0].address;
				
				edited_addr = address_formater(addr);
			})
			.catch((err) => {
				console.log(err);
			});
	}
</script>

<div class="container flex flex-col h-screen max-w-full">

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
