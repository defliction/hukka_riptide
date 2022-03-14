<script>
	import {workSpace} from '../utils/workSpace';
    //import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
    import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
    import GradientText from 'svelte-gradient-typography';
    import { page } from '$app/stores';
    import { Invoice, BillableItem } from '../models'
    import { fetchBillables, billableAuthorFilter, fetchSettings }  from '../api'
    import usdc_svg from '../assets/usdclogo.svg';
    import sPay from '../assets/spayButton.svg';
    import sPayHover from '../assets/spayButtonHover.svg';
    import SvelteCopyUrlButton from 'svelte-copy-url-button';
    import sol_svg from '../assets/spaygradient.svg';
    import solana_png from '../assets/solanaLogo.png';
    import {clusterApiUrl, Connection, PublicKey, sendAndConfirmTransaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
    import QRCodeStyling from '@solana/qr-code-styling'
    import { encodeURL, createQR, parseURL, createTransaction, validateTransactionSignature, findTransactionSignature } from '@solana/pay';
    import BigNumber from 'bignumber.js';

    const isBeta = $page.url.searchParams
    //const isBeta = $page.url.searchParams.has('beta');
    let fromName = ""
    let fromAddress = ""
    let fromTaxRef = ""
    let payeeName = ""
    let payeeAddress = ""
    let payeeTaxRef = ""
    let invoice;
    let populatedItems = []
    let loadingPop = false;
    // If url is not passed it will copy the url of the current page
    let shareUrl = $page.url
    let paymentStatus = 0
    
    let recipient;
    let amount;
    let reference;
    const label = 'hukka.xyz';
    const message = 'marked for future use';
    const memo = 'marked for future use';
    const splToken = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
    let qrCode;
    let url;
    let payButton = sPay;
    $: invoiceAuth = ""
	
    $: if ($walletStore.connected) {
		populateInvoice();
	}
    $: qrCodeTest = createQR(url)._svg;

    $: sum = ($walletStore.connected) ? populatedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.price*currentElement.quantity;
            }, 0 /* initial value of sumSoFar */):0;
    $: totalQuantity = ($walletStore.connected) ? populatedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.quantity;
            }, 0 /* initial value of sumSoFar */):0;
    
    

    async function populateInvoice() {
        loadingPop = true;
        invoice = await $workSpace.program.account.invoice.fetch(isBeta.toString().split("=")[0]);
        //let billables = await fetchBillables({ program: $workSpace.program },[billableAuthorFilter(invoice.author)] )
        //let senderInfo = await fetchSettings({ program: $workSpace.program },[billableAuthorFilter(invoice.author)] )
        let billables = await fetchBillables({ program: $workSpace.program } )
        let senderInfo = await fetchSettings({ program: $workSpace.program } )
        console.log("Invoice info: ", invoice)
        console.log("wallet ", $workSpace.provider.wallet.publicKey , " author ", invoice.author)
        
        if (senderInfo) {
            fromName = senderInfo[0].name.toString()
            fromAddress = senderInfo[0].address.toString()
        }
        else {
            fromName = invoice.author.toString()
        }
        invoiceAuth = invoice.author.toString()
        let payeeInfo = JSON.parse(invoice.addressee)
        let items = JSON.parse(invoice.items)
        let quantities = JSON.parse(invoice.quantities)
        payeeName = payeeInfo[0]
        payeeAddress = payeeInfo[1]
        payeeTaxRef = payeeInfo[2]
   
        populatedItems = []
        for (var item in items) {
            let billItem = new BillableItem(billables[item].publicKey, billables.find(element => element.akey == items[item]))
            billItem.quantity = quantities[item]
            populatedItems.push(billItem)
        }
        populatedItems = populatedItems
        
        let recipient = invoice.author;
        let amount = new BigNumber(numberWithCommas(populatedItems.reduce(function(sumSoFar, currentElement) {
                return sumSoFar + currentElement.price*currentElement.quantity;
            }, 0 /* initial value of sumSoFar */),2));
        let reference = new PublicKey(invoice.paymentref);
        console.log(reference)
        url = ($walletStore.connected) ? encodeURL({ recipient, amount, splToken, reference, label, message, memo }) : null;
        try {
            qrCode = createQR(url)._svg;
            console.log("Qr ", qrCode)
        }
        catch (e) {
            qrCode._svg = ""
            console.log("error making QR ", e)
        }
        
        
        let status;
        let amountPaid;
        try {
            const signatureInfo = await findTransactionSignature($workSpace.provider.connection, reference, undefined, 'confirmed');
            //console.log(signatureInfo)
            status = await validateTransactionSignature($workSpace.provider.connection, signatureInfo.signature, recipient, amount, undefined, reference, "confirmed");
            //invoiceIn.pmt = status
            amountPaid = status.meta.preBalances[0] - status.meta.postBalances[0] - status.meta.fee
            console.log("cehck here NOW ", amountPaid, amount, sum)
            if (numberWithDecimals(sum,2) == amountPaid/LAMPORTS_PER_SOL) {
                paymentStatus = 1
               //return [1,amountPaid/LAMPORTS_PER_SOL]
            }
            else {
                paymentStatus = 0
                //return [0,amountPaid/LAMPORTS_PER_SOL]
            }
        
        }
        catch (e) {
            console.log("error check pmt ", e)
            paymentStatus = 0
            loadingPop = false
            //return [0,0]
        }
        loadingPop = false;

    }

    async function makePayment() {
        console.log("mk pmt ", invoice)
        let payer = new PublicKey($workSpace.provider.wallet.publicKey)
        let recipient = new PublicKey(invoice.author)
        let amount = new BigNumber(numberWithDecimals(sum,2))
        let ref = new PublicKey(invoice.paymentref)
        console.log("mking pmt ", amount, sum, payer)
        //console.log (test1, test2)
        //to insert SPL TOKEN TO USDC!
        
        const tx = await createTransaction($workSpace.connection, payer, recipient, amount, {
                    reference:ref,
                    memo,
                    });
        let payment = await $workSpace.provider.send(tx)
        
        console.log("Payment ", payment, ref)
        populateInvoice()
        //sendAndConfirmTransaction($workSpace.connection, tx, [$walle]);
    }
   
    function numberWithCommas(x, int) {
        return x.toFixed(int).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    }
    function numberWithDecimals(x, int) {
        return x.toFixed(int)
    }
    //{#if invoiceAuth == $workSpace.provider.wallet.publicKey.toString()}
</script>
{#if !$walletStore?.connected}
    Please connect wallet to view Invoice.
{:else}
{#if loadingPop}
<div class = "flex p-4 justify-center flex-wrap">
    <div class="indicator">
    <span class="indicator-item indicator-center indicator-middle badge badge-secondary">Loading...</span> 
   
  </div>
</div>
  
{/if}
{#if paymentStatus == 1}
<div class = "flex p-4 justify-center flex-wrap">
    
    <GradientText size="35px">Invoice fully paid!</GradientText>
</div>
{:else if invoiceAuth == $workSpace.provider.wallet.publicKey.toString()}
<div class = "flex p-4 justify-center flex-wrap">
    <GradientText size="35px">View Invoice</GradientText>
    <div class = "flex justify-center flex-wrap basis-full">
       <p class="align-middle">Invoice submitted successfully!</p>
    </div>
    <p class="align-middle">Share the link with the recipient who will then have the option to settle using  <img class="pl-1 align-middle max-h-5" style='display:inline' src="{sol_svg}"/></p>
</div>
<div class = "flex p-4 justify-center flex-wrap">
    <ul class="steps">
        <li class="step step-primary">Create</li>
        <li class="step step-primary">Save</li>
        <li class="step step-primary text-accent"><SvelteCopyUrlButton
            class=" flex fill-accent"
            size="17"
            defaultText="Share"
            copiedText="Copied!"
            icon="{false}"
            timeout="1000"
            url="{shareUrl}"
            /></li>
        <li class="step">Receive payment</li>
      </ul>
      
</div>
{:else}
<div class = "flex p-4 justify-center flex-wrap">
    
    <GradientText size="35px">View & Pay Invoice</GradientText>
</div>
<div class = "flex pb-2 justify-center flex-wrap">

    <img class="btn btn-link align-center" on:mouseenter={() => payButton=sPayHover} on:mouseleave={() => payButton=sPay} on:click={makePayment} src="{payButton}"/>
    {#if qrCode}
    <qrCode/>

   
    {/if}
    
</div>

{/if}

<div class = "flex justify-center flex-wrap">
    
    <div class="w-1/3 pr-2 pb-2">
        <div class="card bg-base-200 shadow-md ">
            <div class="card-body">
                <GradientText size="20px">From</GradientText>
                <strong class="text text-sm p-0">Name</strong>
                <t class="text text-sm p-0">{fromName}</t>
                <strong class="text text-sm p-0">Address</strong>
                <t class="text text-sm p-0">{fromAddress}</t>
                <strong class="text text-sm p-0">Tax reference</strong>
                <t class="text text-sm p-0">VAT 98-7654321</t>
             
            </div>
        </div>
    </div>
    {#if payeeName !="Anon"}
    <div class="pl-2 w-1/3 pb-2">
        <div class=" card bg-base-200 shadow-md">
            <div class="card-body ">
                <GradientText size="20px">To</GradientText>
                <strong class="text text-sm p-0">Name</strong>
                <span class="text text-sm p-0">{payeeName}</span>
                <strong class="text text-sm p-0">Address</strong>
                <t class="text text-sm p-0">{payeeAddress}</t>
                <strong class="text text-sm p-0">Tax reference</strong>
                <t class="text text-sm p-0">{payeeTaxRef}</t>
            </div>
        </div>
    </div>
    {/if}
</div>
<div class="flex justify-center">
    <div class="p-2 w-2/3 ">
        <div class=" card bg-base-200 shadow-md align-center ">
            <div class="card-body ">
                <GradientText size="20px">Summary</GradientText>
                <p> {numberWithCommas(totalQuantity)} items </p>
                <GradientText size="20px"><img class="h-5 align-top mask mask-circle" style='display:inline' src="{solana_png}"/>  {numberWithCommas(sum,2)}</GradientText>
                <div class="pt-4">
                <GradientText size="16px">Detail</GradientText>
                <div class="overflow-x-auto">
                    <table class="table table-compact w-full">
                    <thead>
                        <tr>
                        <th>Description</th> 
                        <th class="text-right pr-6">Price</th> 
                        <th class="text-right pr-6">Quantity</th> 
                        </tr>
                    </thead> 
                    <tbody>
                        {#each populatedItems as item}
                        <tr>
                        <td>{item.description}</td> 
                        <td class="text-right pr-6">{numberWithCommas(item.price,2)}</td> 
                        <td class="text-right pr-6">{numberWithCommas(item.quantity,0)}</td>
                        </tr>
                    {/each}
                    </tbody> 
                    </table>
                </div>
                </div>
            </div>
        </div>
        
    </div>
</div>
{/if}
    

