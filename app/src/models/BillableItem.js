export class BillableItem //AccDescription
{ 
    constructor (publicKey, descriptionData) {
        console.log("descript ", descriptionData)
        this.publicKey = publicKey
        this.author = descriptionData.author
        this.akey = (descriptionData.akey) ? descriptionData.akey : descriptionData.key
        this.description = descriptionData.description
        this.price = descriptionData.price
        this.showEdit = false;
        this.edit = false;
        this.quantity = 0;
        
    }
    
    get key () {
        return this.publicKey.toBase58()
    }

}
