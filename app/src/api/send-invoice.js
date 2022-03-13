import { web3 } from '@project-serum/anchor'
import { Invoice } from '../models'

export const sendInvoice = async ({ wallet, program }, items, quantities, addressee) => {
    const invoice = web3.Keypair.generate()
    const desc = web3.Keypair.generate() //payment ref

    await program.rpc.sendInvoice(desc.publicKey.toString(), items, quantities, addressee, {
        accounts: {
            author: wallet.publicKey,
            invoice: invoice.publicKey,
            systemProgram: web3.SystemProgram.programId,
        },
        signers: [invoice]
    })
    
    const invoiceAccount = await program.account.invoice.fetch(invoice.publicKey)
    return new Invoice(invoice.publicKey, invoiceAccount)
}
