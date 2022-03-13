import { web3 } from '@project-serum/anchor'
export const updateInvoice = async ({ wallet, program }, invoice, desc, date, items, quantities) => {
    await program.rpc.updateInvoice(desc, date, items, quantities, {
        accounts: {
            author: wallet.publicKey,
            invoice: invoice.publicKey,
        },
    })

    invoice.description = desc;
    invoice.timestamp = date;
    invoice.items = items;
    invoice.quantities = quantities;

    
}
