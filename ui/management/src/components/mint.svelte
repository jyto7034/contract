<script lang="ts">
	import { mint } from '../lib/helpers.ts'
	import { onMount, afterUpdate } from 'svelte';
	let amount = "";

	async function _mint(){
		mint(amount)
		.then((log) => {
			showAlert = true;
			console.log(log);
		})
		.catch((log) => {
			console.log(log);
		})
	}

	let showAlert = false; // 알림창 표시 상태

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
		<p class="mt-10">amount</p>
		<input class="text-black text-right" bind:value={amount}/>
		<button
			class="h-full mt-10 text-white text-2xl font-bold px-10 button-effect"
			on:click={_mint}
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