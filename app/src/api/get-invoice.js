import { Invoice } from '../models'

export const getInvoice = async ({ program }, publicKey) => {
    const account = await program.value.account.invoice.fetch(publicKey);
    return new Invoice(publicKey, account)
}
