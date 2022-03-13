<script>
	import {workSpace} from '../utils/workSpace';
    //import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
    import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
    import GradientText from 'svelte-gradient-typography';
    import { page } from "$app/stores";	
    import { setupInfo, billables, invoicedItems } from '../stores.js';
    import usdc_svg from '../assets/usdclogo.svg';
    import solana_png from '../assets/solanaLogo.png';
    import { sendInvoice, fetchInvoices }  from '../api'
    import { goto } from '$app/navigation';
    import { invoices } from '../stores.js';


    let payeeName = ""
    let payeeAddress = ""
    let payeeTaxRef = ""
    let submitted = false

    let selectedBillable;
    //let invoicedItems = [];
    $: sum = $invoicedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.price*currentElement.quantity;
            }, 0 /* initial value of sumSoFar */);
    $: totalQuantity = $invoicedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.quantity;
            }, 0 /* initial value of sumSoFar */);
    
    function addItem() {
        try {
            let search = $invoicedItems.find(element => element.akey == selectedBillable.akey)
            if(search) {
                search.quantity += 1
            }
            else {
                selectedBillable.quantity = 1
                $invoicedItems.push(selectedBillable);
            }
            $invoicedItems = $invoicedItems;
        }
        catch (e) {
            console.log("Couldn't add item ", e)
        }
        console.log($invoicedItems)
        
       
	}
    function removeItem(billableIn) {
        //let search = $invoicedItems.find(element => element.akey == billableIn.akey)
        
        $invoicedItems =  $invoicedItems.filter(function(ele){ 
            return ele.akey != billableIn.akey; 
        });
        billableIn.showEdit = false;
        
	}
    async function submitInvoice() {
        let items = []
        $invoicedItems.forEach(element => {
                    items.push(element.akey);
                    });
        let quantities = []
        $invoicedItems.forEach(element => {
                    quantities.push(element.quantity);
                    });
        if ($walletStore.connected) {
            try {
                submitted = true
                let name = (payeeName == "") ? "Anon":payeeName
                let addressee = JSON.stringify([name,payeeAddress,payeeTaxRef])
                let newInvoice = await sendInvoice({wallet: $workSpace.provider.wallet, program: $workSpace.program}, JSON.stringify(items).toString(), JSON.stringify(quantities).toString(),addressee.toString())
                $invoices = await fetchInvoices({ program: $workSpace.program })
                $invoicedItems = []
                goto("/ViewInvoice?"+newInvoice.publicKey.toString())
               
            } 
            catch (err) {
                submitted = false
                console.log('Transaction error: ', err);
            }
	    }
        //console.log("Submtit")
        //console.log(items)
	}
    //$: test = $invoicedItems.sort((a, b) => (a.description < b.description ? -1 : 1))
    function numberWithCommas(x, int) {
        return x.toFixed(int).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    }

</script>
<div class = "flex p-4 justify-center flex-wrap">
    <GradientText size="35px">Create a new invoice</GradientText>
</div>
<div class = "flex p-4 justify-center flex-wrap">
    <ul class="steps">
        <li class="step step-primary">Create</li>
        {#if submitted}
        <li class="step step-primary">Submit</li>
        {:else}
        <li class="step">Submit</li>
        {/if}
        
        <li class="step">Share</li>
        <li class="step">Receive Payment</li>
      </ul>
</div>
<div class = "flex flex-wrap">
    
    <div class="p-4 flex-auto md:w-64 sm:w-48">
        <div class="card bg-base-200 shadow-md">
            <div class="card-body">
                <GradientText size="20px">Billing details</GradientText>
                <div class="indicator px-4 py-2">
                    <span class="indicator-item badge badge-secondary">optional</span>
                  </div>
                <p class="text-xs pb-2">Complete optional billing details of the invoice recipient</p>
                <div class="form-control w-full max-w-xs">
                    <label class="label">
                      <span class="label-text">Name</span>
                    </label>
                    <input type="text" bind:value={payeeName} placeholder="e.g. Sol BigBrain" class="input input-primary input-bordered w-full max-w-xs">
                    <label class="label">
                        <span class="label-text">Physical address</span>
                    </label>
                    <input type="text" bind:value={payeeAddress} placeholder="88 Solana Beach Drive, Milky Way" class="input input-primary input-bordered w-full max-w-xs">
                    <label class="label">
                    <span class="label-text">Tax reference</span>
                    </label>
                    <input type="text" bind:value={payeeTaxRef} placeholder="e.g. VAT 12-3456789" class="input input-primary input-bordered w-full max-w-xs">
                  </div>
            </div>
        </div>
    </div>
    <div class="p-4 flex-auto md:w-96 sm:w-96">
        <div class="card bg-base-200 shadow-md">
            <div class="card-body">
                <h2 class="card-title"><GradientText size="20px">Billables</GradientText></h2>
                <p class="text-xs">Enter custom billable items or use pre-populated entries. To save items visit the Setup page.</p>
                <div class="justify-start card-actions">
                    <div class="form-control">
                        <div class="input-group py-2">
                            <select bind:value={selectedBillable} class="select w-64 select-sm select-primary select-bordered">
                                <option disabled selected>Pick item</option>
                                {#each $billables.sort((a, b) => (a.description < b.description ? -1 : 1)) as billable}
                                    <option value={billable}>{billable.description}</option>
                                {/each}
                            </select>
                            <button class="btn btn-sm" on:click={addItem}>Add</button>
                        </div>
                        <div class="input-group py-2">
                            <input type="text" disabled placeholder="Enter custom item" class="input w-40 input-sm input-primary input-bordered">
                            <input type="text" disabled placeholder="Price" class="input w-24 input-sm input-primary input-bordered">
                            <button class="btn btn-sm" disabled>Add</button>
                        </div>
                    </div>
                </div>
                <div class="overflow-x-auto">
                    <table class="table table-compact w-full">
                    <thead>
                        <tr>
                        <th class="w-48">Description</th> 
                        <th class="w-16">Price</th> 
                        <th class="w-12">Quantity</th> 
                        <th></th>
                        </tr>
                    </thead> 
                    <tbody>
                        {#each $invoicedItems as item}
                            <tr on:mouseenter={()=> item.showEdit = true}
                                on:mouseleave={() => item.showEdit = false}>
                            <td>{item.description}</td> 
                            <td class="text-right pr-6">{numberWithCommas(item.price,2)}</td> 
                            <td class="text-right pr-6">{numberWithCommas(item.quantity,0)}</td>
                            {#if item.showEdit}
                            <button class="btn btn-sm" on:click={() => removeItem(item)}>X</button>
                            {/if}
                            </tr>
                        {/each}
                    </tbody> 
                    </table>
                </div>
            </div>
        </div>
    </div>
    <div class="p-4 flex-none md:w-48 sm:w-48">
        <div class="card bg-base-200 shadow-md">
            <div class="card-body">
                <h2 class="card-title"><GradientText size="20px">Total</GradientText></h2>
                <p> {numberWithCommas(totalQuantity)} items </p>
                <h2 class="justify-center"><GradientText size="20px"><img class="h-5 align-top mask mask-circle" style='display:inline' src="{solana_png}"/> {numberWithCommas(sum,2)}</GradientText></h2>
           </div>
           <a 
                class="btn btn-primary btn-md"
                sveltekit:prefetch
                on:click={() => submitInvoice()}>Submit</a>
        </div>
        
    </div>
    
</div>

    

