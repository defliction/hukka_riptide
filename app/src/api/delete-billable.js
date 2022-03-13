import { writable } from 'svelte/store';
import { Setup } from '../models'

export const deleteBillable = async ({ wallet, program }, billableIn) => {
   
    await program.rpc.deleteBillable({
        accounts: {
            author: wallet.publicKey,
            billable: billableIn.publicKey,
        },
        
    })

    return
}
