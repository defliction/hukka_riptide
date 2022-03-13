import { web3 } from '@project-serum/anchor'
import { BillableItem } from '../models'

export const sendBillable = async ({ wallet, program }, key, desc, price) => {
    console.log("new billable ", key, desc, price)
    const descriptor = web3.Keypair.generate()

    await program.rpc.sendBillable(key, desc, price, {
        accounts: {
            author: wallet.publicKey,
            billable: descriptor.publicKey,
            systemProgram: web3.SystemProgram.programId,
        },
        signers: [descriptor]
    })

    const descriptorAccount = await program.account.billable.fetch(descriptor.publicKey)
    return new BillableItem(descriptor.publicKey, descriptorAccount)
}
