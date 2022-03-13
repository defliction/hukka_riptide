import { web3 } from '@project-serum/anchor'

export const updateSettings = async ({ wallet, program }, setup, n, addr, b_currency) => {
    console.log("trying to update ", n, addr, b_currency)
    await program.rpc.updateSettings(n, addr, b_currency, {
        accounts: {
            author: wallet.publicKey,
            settings: setup.publicKey,
        },
    })

    setup.name = n;
    setup.address = addr;
    setup.base_currency = b_currency;
    
}
