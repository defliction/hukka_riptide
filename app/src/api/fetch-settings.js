import { Setup } from '../models'
import bs58 from 'bs58'

export const fetchSettings = async ({ program }, filters = []) => {
    const settings = await program.account.settings.all(filters);
    return settings.map(descriptor => new Setup(descriptor.publicKey, descriptor.account))
}


