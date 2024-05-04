<script lang="ts">
	import { send } from '../lib/helpers.ts'
	import config from "../../config.json"
	import { onMount, afterUpdate } from 'svelte';

	let address = "";
	let amount = "";
	let to_contract = false;
	let showAlert = false; 

	async function _send(){
		if(to_contract){
			address = config.contract_address;
		}
		console.log(to_contract);
		send(address, amount, to_contract)
		.then((log) => {
			showAlert = true;
			console.log(log);
		})
		.catch((log) => {
			console.log(log);
		})
	}

	afterUpdate(() => {
		if (showAlert) {
			setTimeout(() => {
				showAlert = false;
			}, 4000);
		}
	});
</script>

<div class="content-container h-[730px] w-[1200px] mt-7 outline outline-4 outline-offset-2">
	<div class="content m-10 flex flex-col items-center">
		<input type="checkbox" bind:checked={to_contract} />
		{#if to_contract}
			<p class="mt-10">amount</p>
			<input class="text-black text-right" bind:value={amount}/>
		{:else}
			<p>address</p>
			<input class="text-black text-right" bind:value={address}/>
			<p class="mt-10">amount</p>
			<input class="text-black text-right" bind:value={amount}/>
		{/if}
		<button
			class="h-full mt-10 text-white text-2xl font-bold px-10 button-effect"
			on:click={_send}
		>
			Send
		</button>
		{#if showAlert}
			<div class="alert">
				Please connect your wallet to proceed.
			</div>
		{/if}
	</div>
</div>

<style>
	.alert {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		padding: 20px;
		background-color: gray;
		color: white;
		border-radius: 8px;
		text-align: center;
		z-index: 100;
		transition: opacity 1s ease;
		opacity: 1;
		animation: fadeOut 1s 1s forwards;
	}
	@keyframes fadeOut {
		from { opacity: 1; }
		to { opacity: 0; }
	}
</style>