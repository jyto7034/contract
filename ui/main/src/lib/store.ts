import { writable } from 'svelte/store';
import { connect, type WalletConnect } from '@sei-js/core';

export const _nft_list = writable([]);
export const _token_list = writable([]);

export const _data_cached = writable(false);
export const _is_connect = writable(false);
export const _address = writable('');

export const _wallet = writable<any>(null);