<script>
	import {workSpace} from '../utils/workSpace';
	//import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
    import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { fade } from 'svelte/transition';
	import { LinkedChart, LinkedLabel, LinkedValue } from "svelte-tiny-linked-charts"
	import Reports from './Reports.svelte'

	import { invoices } from '../stores.js';
	
	$: unpaid = $invoices.filter(item => item.pmtState == 0);

	
	function numberWithDecimals(x, int) {
        return x.toFixed(int)
    }
</script>

<div class = "flex justify-center flex-wrap">
	<div class = "p-2">
		{#if $walletStore?.connected && false }
			<div class="alert shadow-lg alert-success " transition:fade="{{duration: 1000}}">
				<div>
					<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					<span>You are connected to the hukka Riptide demo on Devnet! {$workSpace.program.programId}</span>
				</div>
			</div>
		{:else if !$walletStore?.connected}
			<div class="alert shadow-lg alert-warning">
				<div>
					<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
					<span>Your wallet is not connected.</span>
				</div>
			</div>
		{/if}
	</div>
	<div class = "p-2 flex justify-center basis-full ">
	{#if $walletStore?.connected}
		<div class="shadow bg-neutral stats">
			
			<div class="stat place-items-center">
			<div class="stat-title">Total invoices</div>
			<div class="stat-value text-accent">{$invoices.length}</div>
			<div class="stat-desc text-accent"></div>
			</div>
			
			<div class="stat place-items-center">
			<div class="stat-title">Awaiting payment</div>
			<div class="stat-value">{unpaid.length}</div>
			<div class="stat-desc"></div>
			</div>
			
		</div>
	{/if}
	</div>

</div>	
{#if $walletStore?.connected}
	<Reports/>
{/if}