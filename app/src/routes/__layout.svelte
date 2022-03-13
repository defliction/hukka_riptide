<script>
    import "../app.css";
    import { onMount } from 'svelte';
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { WalletProvider, WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import AnchorConnectionProvider from '../utils/AnchorConnectionProvider.svelte';
	//import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
	import {workSpace} from '../utils/workSpace';
	import { clusterApiUrl } from '@solana/web3.js';
	import idl from '../assets/solinvoice.json';
	import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
	import { page } from "$app/stores";	
	import GradientText from 'svelte-gradient-typography';
	import sol_svg from '../assets/spaygradient.svg';
	import { setupInfo, billables, invoices,initializing } from '../stores.js';
	import { fetchSettings, fetchBillables, fetchInvoices, }  from '../api';

	const localStorageKey = 'walletAdapter';
	const network = clusterApiUrl('devnet'); // localhost or mainnet
	//const network = 'http://127.0.0.1:8899';

	let wallets;

	onMount(async () => {
		const { PhantomWalletAdapter, SolflareWalletAdapter } = await import(
			'@solana/wallet-adapter-wallets'
		);

		wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];
	});

	$: if ($walletStore.connected) {
		intializeUser();
		
	}
	async function intializeUser() {
		if ($walletStore.connected) {
			try {
				$initializing = 0
				$setupInfo = await fetchSettings({ program: $workSpace.program })
				$initializing = 15
				console.log("setup info ", $setupInfo)
				$billables = await fetchBillables({ program: $workSpace.program })
				$initializing = 45
				console.log("billables ", $billables)
				$invoices = await fetchInvoices({ program: $workSpace.program })
				$initializing = 100
			
				//console.log("invoices ", $invoices)
			} catch (err) {
				console.log('Transaction error: ', err);
			}
		}
	}
	

</script>
<WalletProvider {localStorageKey} {wallets} autoConnect />
<AnchorConnectionProvider {network} {idl} />
<div class="md:container md:mx-auto 2xl:px-24 xl:px-8">
	<div class="navbar bg-neutral md:mt-4 md:mb-2 shadow-md rounded-box">
		<div class="navbar-start">
			<div class="dropdown lg:hidden">
				{#if $walletStore?.connected}
				{#if $initializing!=100}
					<div class="radial-progress bg-primary text-primary-content border-4 border-primary" style="--value:{initializing};">{$initializing}</div>
				{/if}
				
				<label class="btn btn-ghost btn-circle">
					<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg>
				</label>
				<ul tabindex="0" class="mt-3 p-2 shadow menu menu-compact dropdown-content bg-base-100 rounded-box w-52">
					<li>
						<a href="NewInvoice"
						sveltekit:prefetch
						aria-current={$page.url.pathname === "NewInvoice" ? "page" : undefined}>New</a>
					</li>
				
					<li>
						<a href="Setup"
						sveltekit:prefetch
						aria-current={$page.url.pathname === "Setup" ? "page" : undefined}>Setup</a>
					</li>
				</ul>
				{/if}
			</div>
			
			<a class="btn btn-ghost normal-case text-xl align-middle"
			href="/"
			sveltekit:prefetch
			aria-current={$page.url.pathname === "/" ? "page" : undefined}>  <GradientText style="line-height:1.8;vertical-align:middle" size="25px">hukka</GradientText></a>
		</div>
		<div class="navbar-center hidden lg:flex">
			
				
		
			{#if $walletStore?.connected}
			<ul class="menu menu-horizontal p-0">
		
				<li>
					<a href="NewInvoice"
					sveltekit:prefetch
					aria-current={$page.url.pathname === "NewInvoice" ? "page" : undefined}>Create</a>
				</li>
	
				<li>
					<a href="Setup"
					sveltekit:prefetch
					aria-current={$page.url.pathname === "Setup" ? "page" : undefined}>Settings</a>
				</li>
			</ul>
			{/if}
		</div>
		<div class="navbar-end">
			<WalletMultiButton />
		</div>
	</div>
	
	<slot />
	
	
	<div class="flex justify-center pt-4">
		<p class="text-sm">Built by the hukka Protocol. Powered by <img class="h-5 align-top" style='display:inline' src="{sol_svg}"/></p>
	</div>
</div>
