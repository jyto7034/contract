<script lang="ts">
	import { home_descriptions } from '../lib/descriptions.ts'
	import { createEventDispatcher } from 'svelte';
	
	const dispatch = createEventDispatcher();
</script>

<div class="content-container h-[730px] w-[1200px] mt-7 outline outline-4 outline-offset-2">
	<div class="content m-10 pl-10">
		<div class="content-welcome flex flex-row justify-between">
			<p class="text-3xl">Welcome to chickegg</p>
			<div class="w-[500px] flex flex-row justify-around">
				<div
					class="hover:cursor-pointer px-5 py-4 bg-gray-50 text-black text-2xl transition duration-200 hover:bg-gray-400"
				>
					<button on:click={() => window.location.href = '/swap'}>Buy $EGG</button>
					<div class="px-10 bg-black py-[1px]"></div>
				</div>
				<div
					class="hover:cursor-pointer px-5 py-4 bg-gray-50 text-black text-2xl transition duration-200 hover:bg-gray-400"
				>
					<button>Buy NFT</button>
					<div class="px-10 bg-black py-[1px]"></div>
				</div>
			</div>
		</div>
		<div class="home_descriptionsription-intro mt-6">
			<p class="text-3xl">The first hybrid NFT x Token on SEI</p>
		</div>
		<div class="home_descriptionsription mt-6">
			<p class="text-xl">What is HYBRID?</p>
			{#each home_descriptions as str}
				<p class="mt-3 text-xl">{@html str}</p>
			{/each}
		</div>
	</div>
</div>
