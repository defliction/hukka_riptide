export class Setup 
{
    constructor (publicKey, setupData) {
        this.publicKey = publicKey
        this.author = setupData.author
        this.name = setupData.name
        this.address = setupData.address
        this.base_currency = setupData.baseCurrency
        
    }

    get key () {
        return this.publicKey.toBase58()
    }

}
