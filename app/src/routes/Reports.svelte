<script>
	import {workSpace} from '../utils/workSpace';
    //import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
    import { onMount, onDestroy } from 'svelte';
    import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
    import GradientText from 'svelte-gradient-typography';
    import { invoices, billables } from '../stores.js';
    import { fetchInvoices, deleteInvoice, invoiceAuthorFilter }  from '../api';
    import { page } from "$app/stores";	
    import { goto } from '$app/navigation';
    import { BillableItem } from '../models'
    import BigNumber from 'bignumber.js';
    import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
    import { findTransactionSignature, validateTransactionSignature } from '@solana/pay';
    import usdc_svg from '../assets/usdclogo.svg';

    console.log($invoices)
 
    $: if ($walletStore.connected) {
		checkPayments();
		
	}
    const interval = setInterval(async () => {
        if ($walletStore.connected) {
            checkPayments();
        }
        
    }, 1000);

    onMount(async () => {
        if ($walletStore.connected) {
            checkPayments();
        }
        
    });
    onDestroy(() => clearInterval(interval));
    
    async function checkPayments() {
        for (var item in $invoices) {
            if ($invoices[item].loadingPmt == true) {
                continue
            }
            console.log("chcp ", item)
            let statu = await checkPayment($invoices[item])
            console.log("statu ", statu)
            $invoices[item].pmtState = statu[0]
            $invoices[item].pmtAmount = statu[1]
            $invoices[item].invAmount = statu[2]
        
        }
        $invoices = $invoices
    }
   
    async function deleteBillableItem(invoiceIn) {
        if ($walletStore.connected) {
            try {
                console.log(invoiceIn.publicKey)
                let deleted = await deleteInvoice({wallet: $workSpace.provider.wallet, program: $workSpace.program}, invoiceIn)
                $invoices = await fetchInvoices({ program: $workSpace.program }, [invoiceAuthorFilter($workSpace.provider.wallet.publicKey)]) 
            } 
            catch (err) {
                console.log('Transaction error: ', err);
            }
	  }
    }

    async function checkPayment(invoiceIn) {
        //console.log("invoice ", invoiceIn)
       
        let items = JSON.parse(invoiceIn.items)
        
        let quantities = JSON.parse(invoiceIn.quantities)
        let populatedItems = []
        for (var item in items) {
            let billItem = new BillableItem($billables[item].publicKey, $billables.find(element => element.akey == items[item]))
            billItem.quantity = quantities[item]
            populatedItems.push(billItem)
        }
        let sum = populatedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.price*currentElement.quantity;
            }, 0 /* initial value of sumSoFar*/ );
        let sumbn = new BigNumber(numberWithDecimals(sum,2))
        invoiceIn.invAmount = sum
        $invoices = $invoices
        let recipient = new PublicKey(invoiceIn.author)
        let payer = new PublicKey($workSpace.provider.wallet.publicKey)
        
        //let reference = new PublicKey(invoiceIn.paymentref);
        let pref = new PublicKey(invoiceIn.pref)
        //console.log(pref)
        const amountInLamports = sumbn.times(LAMPORTS_PER_SOL).integerValue(BigNumber.ROUND_FLOOR);
        //console.log(sum, sumbn, amountInLamports)
        
        let status;
        let amountPaid;
        try {
            invoiceIn.loadingPmt = true
            const signatureInfo = await findTransactionSignature($workSpace.provider.connection, pref, undefined, 'confirmed');
            //console.log(signatureInfo)
            status = await validateTransactionSignature($workSpace.provider.connection, signatureInfo.signature, recipient, sumbn, undefined, invoiceIn.paymentref, "confirmed");
            //invoiceIn.pmt = status
            amountPaid = status.meta.preBalances[0] - status.meta.postBalances[0] - status.meta.fee
            console.log("amount paid ", amountPaid, sumbn)
            if (numberWithDecimals(sum,2) == amountPaid/LAMPORTS_PER_SOL) {
                invoiceIn.loadingPmt = false
               return [1,amountPaid/LAMPORTS_PER_SOL, numberWithDecimals(sum,2)]
            }
            else {
                invoiceIn.loadingPmt = false
                return [0,amountPaid/LAMPORTS_PER_SOL, numberWithDecimals(sum,2)]
            }
        
        }
        catch (e) {
            console.log("error check pmt ", e)
            return [0,0,numberWithDecimals(sum,2)]
        }
    
        
        //console.log("pmt status ", status, amountPaid
        //return amountPaid
    }
    function numberWithCommas(x, int) {
        return x.toFixed(int).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    }
    function numberWithDecimals(x, int) {
        return x.toFixed(int)
    }
</script>
<div class="flex justify-center pt-4">
    
<div class=" card bg-base-300 shadow-md md:w-3/4">
<div class="overflow-x-auto">
    <table class="table table-compact w-full">
    <thead>
        <tr>
        <th class="pl-4 w-16">Customer</th> 
        <th class="w-16">Reference</th> 
        <th class="w-16">Amount</th> 
        
        <th class="w-16">Status</th>
        
        <th class="w-16"></th>
        </tr>
    </thead> 
    <tbody>
        {#each $invoices as item}
            <tr on:mouseenter={()=> item.showEdit = true}
                on:mouseleave={() => item.showEdit = false}>
            <td class="pl-4 text-left pr-6">{JSON.parse(item.addressee)[0]}</td>
            <td><a href="ViewInvoice?{item.publicKey}"
                sveltekit:prefetch
                aria-current={$page.url.pathname === "ViewInvoice" ? "page" : undefined} 
                class="text-accent" >{item.publicKey.toString().substr(0,4) + "..." + item.publicKey.toString().slice(-4)}</a></td>
            <td class="text-left pr-6">{numberWithCommas(parseFloat(item.invAmount),2)}</td>
            {#if item.pmtState == 1}
            <td class="btn text-xs btn-sm btn-success" on:click={() => goto("/ViewInvoice?"+item.publicKey.toString())}>Paid in Full</td>
            {:else}
            <td class="btn text-xs btn-sm btn-warning " on:click={() => goto("/ViewInvoice?"+item.publicKey.toString())}>Awaiting Payment</td>
            {/if}
            {#if item.showEdit}
            <button class="btn btn-sm" on:click={() => deleteBillableItem(item)}>X</button>
            {/if}
            </tr>
        {/each}
    </tbody> 
    </table>
</div>
</div>  
</div>

