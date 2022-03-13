<script>
	import {workSpace} from '../utils/workSpace';
    import { walletStore } from '@svelte-on-solana/wallet-adapter-core';

    import GradientText from 'svelte-gradient-typography';


</script>

<div class = "flex p-4 flex-wrap text-left">
    <div class = "p-2 basis-full"><GradientText size="35px">About us</GradientText></div>
   
    <p>We aim to provide an invoicing dapp running on Solana Pay rails to keep track and receive payments faster. </p>
    <p>We don't take plan to take a fee on the core invoicing functions and will relentlessly pursue safe and secure payments for all users.</p>
</div>
<div class = "p-2 flex basis-full text-left ">
  
        <div class="card p-8 shadow bg-neutral text-left">
            <GradientText size="35px">Workflow stages</GradientText>
      
              
        <ul class="steps steps-vertical ">
            <li class="step step-primary ">Connect wallet and create new invoice. Your wallet address will be the recipient of the new invoice.</li>
            <li class="step step-primary">Submit invoice - a new 'account' will be created on chain holding the state of the invoice.</li>
            <li class="step step-primary ">Once an invoice is complete it will be verified directly on chain. Once a payment reference is matched the invoice will be marked as paid in the 'Reports' tab as well as on the invoice page itself.</li>
            <li class="step step-primary">Invoices can be deleted to recover Solana account storage rent. </li>
        </ul>

            
        </div>

</div>

