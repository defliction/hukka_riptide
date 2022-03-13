import { web3 } from '@project-serum/anchor'
import { Setup } from '../models'

export const sendSettings = async ({ wallet, program }, n, addr, b_currency, filters = []) => {
    //const setup = await program.value.account.ledger_info.all(filters);
    const setup = await program.account.settings.all();
    var isFirst = false;
    const newAccount = web3.Keypair.generate()
    if (setup.length == 0) {       
        isFirst = true
    }
    
    if (isFirst) {
        console.log("settings: ", n, addr, b_currency )
        await program.rpc.sendSettings(n, addr, b_currency, {
            accounts: {
                author: wallet.publicKey,
                settings: newAccount.publicKey,
                systemProgram: web3.SystemProgram.programId,
            },
            signers: [newAccount]
        })
    }
    else {
        await program.rpc.sendSettingso(n, addr, b_currency, {
            accounts: {
                author: wallet.publicKey,
                settings: setup.publicKey,
                systemProgram: web3.SystemProgram.programId,
            },
        })
    }
    console.log("want to fetch" )
    const setupAccount = await program.account.settings.fetch(newAccount.publicKey)
    console.log("settings ", setupAccount)
    return new Setup(setup.publicKey, setupAccount)
}
