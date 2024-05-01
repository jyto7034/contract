<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
    
    export let swap_type;
    export let result_value= "0.0";
    let currentFunction = 'token_to_sei';
    let value;

	function checkNumber(event) {
		// 숫자, 소수점, 백스페이스, 방향키만 허용
		if (event.key === '.' || event.key === '-' || event.key === 'Backspace' ||
			event.key === 'ArrowLeft' || event.key === 'ArrowRight' ||
			(event.key >= '0' && event.key <= '9')) {
                return true; // 입력 허용
            }
            event.preventDefault(); // 그 외의 입력 차단
            return false;
        }
        
        function test(event){
            dispatch('message', { value: event.target.value });
    }
</script>

<div class="content-container mt-12 h-[100px] w-[800px] outline outline-4 outline-offset-2 flex flex-row items-center">
    <div class="outline outline-4 outline-offset-2 px-5 py-3 rounded ml-16">
        <p class="text-3xl w-12 text-center">{swap_type}</p>
    </div>
    <div class="outline outline-4 outline-offset-2 px-5 py-3 rounded ml-[300PX] mr-10 fixed-size">
        <input class="text-3xl w-full h-full text-white bg-black outline-none text-right hide-spinner" bind:value={result_value} on:keydown={checkNumber} type="number" on:input={test}/>
    </div>
</div>


<style>
	/* 증감 버튼 숨기기 */
	.hide-spinner::-webkit-inner-spin-button,
	.hide-spinner::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
</style>