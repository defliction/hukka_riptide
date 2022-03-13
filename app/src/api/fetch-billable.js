import { BillableItem } from '../models'
import bs58 from 'bs58'

export const fetchBillables = async ({ program }, filters = []) => {
    const descriptions = await program.account.billable.all(filters);
    return descriptions.map(descriptor => new BillableItem(descriptor.publicKey, descriptor.account))
}

export const billableAuthorFilter = authorBase58PublicKey => ({
    memcmp: {
        offset: 8, // Discriminator.
        bytes: authorBase58PublicKey,
    }
})

