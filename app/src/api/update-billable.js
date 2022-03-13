import { web3 } from '@project-serum/anchor'
export const updateBillable = async ({ wallet, program }, bill, desc, pr) => {
    console.log("udpate bllable ", bill, desc, pr)
    bill.price = bill.price.toString()
    await program.rpc.updateBillable(desc, pr, {
        accounts: {
            author: wallet.publicKey,
            billable: bill.publicKey,
        }, 
    })

    bill.description = desc;
    bill.price = pr

}
