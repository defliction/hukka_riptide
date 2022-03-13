import { writable } from 'svelte/store';
import { Setup, BillableItem } from './models'

export let setupInfo = writable(Setup);
export let billables = writable([]);
export let invoices = writable([]);
export let payments = writable([]);
export let invoicedItems = writable([]);
export let currentPage = writable(1);
export let pSize = writable(6);
export let initializing = writable(0);

