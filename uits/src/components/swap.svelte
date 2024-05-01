<script lang="ts">
	import { swap_token_to_sei, swap_sei_to_token } from '../lib/helpers.ts';
	import { _is_connect  } from '../lib/store.ts'

	import Input from './input.svelte';
	import config from "../../config.json"

	let is_connect = false;
	_is_connect.subscribe((value) => {
		is_connect = value;
	});
	
	let swapType1 = "Egg";
	let swapType2 = "sei";

	let swap1_value = "";
	let result_value = "0.0";

	let showAlert = false; // 알림창 표시 상태

	function swap() {
		if (!is_connect) {
			showAlert = true;
			setTimeout(() => { showAlert = false; }, 1000); // 3초 후 알림창 숨기기
			return; // 지갑 연결이 안 되어 있으면 함수 실행 중지
		}
		[swapType1, swapType2] = [swapType2, swapType1]; // swap_type 값 서로 바꾸기
		update_result();
	}

	function confirm() {
		if (!is_connect) {
			showAlert = true;
			setTimeout(() => { showAlert = false; }, 1000); // 3초 후 알림창 숨기기
			return; // 지갑 연결이 안 되어 있으면 함수 실행 중지
		}
		if (swapType1 == "sei"){
			console.log("sei to token" + swap1_value)
			swap_sei_to_token(swap1_value);
		}else{
			console.log("token to sei" + swap1_value)
			swap_token_to_sei(swap1_value);
		}
	}

	function update_result(){
		if (swapType1 == "sei"){
			result_value = swap1_value / config.one_sei * config.exchange_rate;
		}else{
			result_value = swap1_value / config.exchange_rate * config.one_sei;
		}
	}

	function swap1_handler(event){
		swap1_value = event.detail.value;
		update_result();
	}
</script>

<div class="content-container h-[600px] w-[1200px] mt-32 outline outline-4 outline-offset-2">
	<div class="content m-10 pl-10 flex flex-col items-center">
		{#if showAlert}
			<div class="alert">
				Please connect your wallet to proceed.
			</div>
		{/if}
		<Input swap_type={swapType1} on:message={swap1_handler}/>
		<div>
			<button on:click={swap} class="text-3xl pt-12">
				⭡⭣
			</button>
		</div>
		<Input swap_type={swapType2} {result_value}/>
		<div class="mt-12 text-3xl">
			<button on:click={confirm}> 
				Swap
			</button>
		</div>
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
	}
</style>