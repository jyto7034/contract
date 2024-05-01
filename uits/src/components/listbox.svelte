<script lang="ts">
	import { swap_token_to_nft, swap_nft_to_token } from '../lib/helpers.ts';
	import { createEventDispatcher } from 'svelte';

	export let data_list;
	export let button_name: string;
	const dispatch = createEventDispatcher();
	let currentPage = 0;

	$: pageCount = Math.ceil(data_list.length / 4);
	$: paginatedData = data_list.slice(currentPage * 4, (currentPage + 1) * 4);
	$: noDataMessage = "There are no NFTs available for exchange.";
	$: executeFunction = (token_id: string) => {
		if (button_name.includes("NFT")) {
			swap_token_to_nft(token_id);
		} else if (button_name.includes("EGG")) {
			swap_nft_to_token(token_id);
		}
		dispatch('message', { connected: addr });
	};

	if (data_list.length == 0){
		noDataMessage = "You do not own any NFTs.";
	}
</script>

<div class="nft-listbox h-[200px] w-full mt-10">
	{#each paginatedData as item}
		<div class="flex flex-row pt-3 justify-between">
			<p>{item}</p>
			<button class="px-2 py-1 text-black bg-gray-200 transition duration-200 hover:bg-gray-400" on:click={() => executeFunction(item)}>
				Swap to {button_name}
			</button>
		</div>
	{/each}
	{#if data_list.length == 0}
		<p>{noDataMessage}</p>
	{/if}
</div>
{#if pageCount > 1}
<div class="flex flex-row justify-center">
	{#each Array(pageCount) as _, i}
		<button
			class="w-8 h-8 mt-3 ml-3 text-center text-black bg-gray-200 transition duration-200 hover:bg-gray-400"
			on:click={() => currentPage = i}
		>
			{i + 1}
		</button>
	{/each}
</div>
{/if}

