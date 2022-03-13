import dayjs from "dayjs"

export class Invoice
{
    constructor (publicKey, invoiceData) {
        this.publicKey = publicKey
        this.author = invoiceData.author
        this.timestamp = invoiceData.timestamp.toString()
        this.pref = invoiceData.paymentref
        this.items = invoiceData.items
        this.quantities = invoiceData.quantities
        this.showEdit = false;
        this.addressee = invoiceData.addressee;
        this.pmtState = "";
        this.pmtAmount = 0;
        this.invAmount = 0;
        this.loadingPmt = false;

    }

    get key () {
        return this.publicKey.toBase58()
    }

    get author_display () {
        const author = this.author.toBase58()
        return author.slice(0,4) + '..' + author.slice(-4)
    }

    get created_at () {
        return dayjs.unix(this.timestamp).format('lll')
    }

}
