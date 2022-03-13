<script>
	import {workSpace} from '../utils/workSpace';
  //import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
  import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
  import GradientText from 'svelte-gradient-typography';
  import { fetchSettings, sendSettings, deleteSettings, updateSettings, sendBillable, fetchBillables, updateBillable, deleteBillable }  from '../api'
  import { setupInfo, billables } from '../stores.js';

  //tests ledgerInfo update 3 times - link once?
  let setupDetails = $setupInfo[0];
  let name = ($setupInfo[0]) ? $setupInfo[0].name:""
  let address = ($setupInfo[0]) ? $setupInfo[0].address:""
  let base_c = "USDC"
  let editSettings = false;
  let addItem = false;
  let editingItem = false;

  //let itemsMaxKey = $billables.length
  let itemsMaxKey = Math.max(...$billables.map(obj => {return obj.akey}))+1
  let itemName = ""
  let itemPrice = ""

  

  async function deleteUserSettings() {
		if ($walletStore.connected) {
      try {
        $setupInfo[0] = await deleteSettings({wallet: $workSpace.provider.wallet, program: $workSpace.program}, $setupInfo[0])
        $setupInfo = await fetchSettings({ program: $workSpace.program })
        address = ""
        name = ""
      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
	}
  async function updateUserSettings() {
		if ($walletStore.connected && !editSettings) {
      try {
        $setupInfo[0] = await sendSettings({wallet: $workSpace.provider.wallet, program: $workSpace.program}, name, address, base_c)
        $setupInfo = await fetchSettings({ program: $workSpace.program })
        console.log('new settings: ', $setupInfo);
        address = $setupInfo[0].address
        name = $setupInfo[0].name

      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
    else if ($walletStore.connected && editSettings) {
      try {
        $setupInfo[0] = await updateSettings({wallet: $workSpace.provider.wallet, program: $workSpace.program}, $setupInfo[0], name, address, base_c)
        $setupInfo = await fetchSettings({ program: $workSpace.program })
        console.log('new settings: ', $setupInfo);
        address = $setupInfo[0].address
        name = $setupInfo[0].name
        editSettings = false
      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
	}
  async function addNewBillable() {
    if ($walletStore.connected) {
      try {
        let newBillable = await sendBillable({wallet: $workSpace.provider.wallet, program: $workSpace.program}, (itemsMaxKey<0) ? "0":itemsMaxKey.toString(), itemName, itemPrice)
        $billables = await fetchBillables({ program: $workSpace.program })
        itemsMaxKey = Math.max(...$billables.map(obj => {return obj.akey}))+1
        addItem = false
        
      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
  }
  async function updateBillableItem(billableIn) {
    if ($walletStore.connected) {
      try {
        let newBillable = await updateBillable({wallet: $workSpace.provider.wallet, program: $workSpace.program}, billableIn, billableIn.description, billableIn.price.toString())
        console.log('updated item: ', newBillable);
        $billables = await fetchBillables({ program: $workSpace.program })
        itemsMaxKey = Math.max(...$billables.map(obj => {return obj.akey}))+1
        editingItem = false;
        
      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
  }
  async function deleteBillableItem(billableIn) {
    if ($walletStore.connected) {
      try {
        let deleted = await deleteBillable({wallet: $workSpace.provider.wallet, program: $workSpace.program}, billableIn)
        $billables = await fetchBillables({ program: $workSpace.program })
        itemsMaxKey = Math.max(...$billables.map(obj => {return obj.akey}))+1
        editingItem = false;
        
      } 
      catch (err) {
        console.log('Transaction error: ', err);
      }
	  }
  }
  function numberWithCommas(x, int) {
        return x.toFixed(int).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    }
    function numberWithDecimals(x, int) {
        return x.toFixed(int)
    }

</script>

<div class = "flex flex-wrap">
  
    <div class="p-4 flex-auto md:w-64 sm:w-64">
      
        <div class="card bg-base-200 shadow-md">
         
            <div class="card-body">
                <GradientText size="20px">Your trading details</GradientText>
                <div class="indicator px-4 py-2">
                  <span class="indicator-item badge badge-secondary">optional</span>
                </div>
                <p class="text-xs pb-2">This information will appear on each invoice. Unsettled invoices are retrospectively updated.</p>
                <div class="form-control w-full max-w-xs">
                    <label class="label">
                      <strong class="label-text">Trading name</strong>
                    </label>
                      {#if !$setupInfo[0] || editSettings}
                        <input type="text" bind:value={name} placeholder="e.g. Sol Surfboards LLC" class="input input-sm input-primary input-bordered w-full max-w-xs">
                        <label class="label">
                          <span class="label-text-alt">Typically your registered company name</span>
                        </label>
                      {:else}
                        <span class="label-text text-sky-400 pl-2">{$setupInfo[0].name}</span>
                      {/if}
                  </div>
                  <div class="form-control w-full max-w-xs">
                    <label class="label">
                      <strong class="label-text">Physical address</strong>
                    </label>
                    {#if !$setupInfo[0] || editSettings}
                      <input type="text" bind:value={address} placeholder="e.g. 88 Solana Beach Drive, Milky Way" class="input input-sm input-primary input-bordered w-full max-w-xs">
                    {:else}
                      <span class="label-text text-sky-400 pl-2">{$setupInfo[0].address}</span>
                      
                    {/if}
                    
                  </div>
            <div class="justify-end card-actions">
              {#if !$setupInfo[0]}
                <button class="btn btn-sm btn-primary" on:click={updateUserSettings}>Save</button>
              {:else if !editSettings}
                <button class="btn btn-sm btn-primary" on:click={() => editSettings = true}>Edit</button>
              {:else if editSettings}
                <button class="btn btn-sm btn-primary" on:click={updateUserSettings}>Save</button>
                <button class="btn btn-sm btn-outline btn-primary" on:click={() => editSettings = false}>Cancel</button>
              {/if}
              {#if $setupInfo[0] && !editSettings}
                <button class="btn btn-sm btn-outline btn-secondary" on:click={deleteUserSettings}>Delete</button>
              {/if}
            </div>
            </div>
        </div>
    </div>
    <div class="p-4 flex-auto md:w-96 sm:w-96">
        <div class="card bg-base-200 shadow-md">
            <div class="card-body">
            <h2 class="card-title"><GradientText size="20px">Saved billables</GradientText></h2>
            <p class="text-xs pb-2">List of billable items or inventory for quick use on new invoice creation. Unsettled invoices are retrospectively updated.</p>
            <div class="justify-start card-actions">
              {#if !addItem}
                <button class="btn btn-sm btn-primary" on:click={() => addItem = true}>New</button>
              {:else}
                <div class="lg:input-group ">
                  <input type="text" bind:value={itemName} placeholder="Enter custom item" class="input w-48 input-sm input-primary input-bordered">
                  <input type="text" bind:value={itemPrice} placeholder="Price" class="input w-24 input-sm input-primary input-bordered">
                  <button class="btn btn-primary btn-sm" on:click={addNewBillable}>Add</button>
                  <button class="btn btn-outline btn-primary btn-sm" on:click={() => addItem = false}>Cancel</button>
                </div>
              {/if}
            </div>
            <div class="overflow-x-auto">
                <table class="table table-compact w-full">
                  <thead>
                    <tr>
                     
                      <th class="w-48">Description</th> 
                      <th class="w-24 text-right">Price</th> 
                      <th class="w-12"></th>
                    </tr>
                  </thead> 
                  <tbody>
                    {#each $billables.sort((a, b) => (a.description < b.description ? -1 : 1)) as billable}
                    <tr on:mouseenter={()=> (!editingItem) ? billable.showEdit = true:billable.showEdit = false}
                        on:mouseleave={() => billable.showEdit = false}>
                      {#if !billable.showEdit && !billable.edit }
                       
                        <td class="text-sky-400">{billable.description}</td> 
                        <td class="text-sky-400 text-right pr-6">{numberWithCommas(billable.price,2)}</td> 
                      {:else if billable.showEdit && !billable.edit}
                     
                        <td class="text-sky-400">{billable.description}</td> 
                        <td class="text-sky-400 text-right pr-6">{numberWithCommas(billable.price,2)}</td> 
                        <button class="w-10 btn btn-primary btn-sm" on:click={() => (billable.edit = true, editingItem = true)}>Edit</button>
                      {:else if billable.edit}
                      
                        <td><input type="text" bind:value={billable.description} class="input w-48 input-sm input-primary input-bordered "></td>
                        <td><input type="text " bind:value={billable.price} class="input w-24 input-sm input-primary input-bordered text-right "></td>
                        <td><button class="btn btn-primary btn-sm" on:click={() => updateBillableItem(billable)}>Save</button>
                        <button class="btn btn-outline btn-secondary btn-sm" on:click={() => deleteBillableItem(billable)}>Del</button>
                        <button class="btn btn-outline btn-primary btn-sm" on:click={() => (billable.edit = false,billable.showEdit = false, editingItem=false)}>X</button></td>
                        
                      {/if}
                    </tr>
                    {/each}
                  </tbody> 
                </table>
              </div>
            
            </div>
        </div>
    </div>
</div>



