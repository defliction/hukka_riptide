import { Invoice } from '../models'
import bs58 from 'bs58'

export const fetchInvoices = async ({ program }, filters = []) => {
    const invoices = await program.account.invoice.all(filters);
    return invoices.map(invoice => new Invoice(invoice.publicKey, invoice.account))
}

export const invoiceAuthorFilter = authorBase58PublicKey => ({
    memcmp: {
        offset: 8, // Discriminator.
        bytes: authorBase58PublicKey,
    }
})
//not fixed yet
export const invoicePublicKeyFilter = topic => ({
    memcmp: {
        offset: 8 + // Discriminator.
            32 + // Author public key.
            8 + // Timestamp.
            4, // Topic string prefix.
        bytes: bs58.encode(Buffer.from(topic)),
    }

})


