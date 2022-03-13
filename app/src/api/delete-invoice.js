export const deleteInvoice = async ({ wallet, program }, inv) => {
    await program.rpc.deleteInvoice({
        accounts: {
            author: wallet.publicKey,
            invoice: inv.publicKey,
        },
    })
}
