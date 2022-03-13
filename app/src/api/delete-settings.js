import { writable } from 'svelte/store';
import { Setup } from '../models'

export const deleteSettings = async ({ wallet, program }, settings) => {
   
    await program.rpc.deleteSettings({
        accounts: {
            author: wallet.publicKey,
            settings: settings.publicKey,
        },
        
    })

    return
}
