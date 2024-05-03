import { connect, type WalletConnect } from '@sei-js/core';

export const connectWallet = (name: any) => {
	return connect(name, 'atlantic-2')
		.then((wallet: WalletConnect) => {
			return wallet;
		})
		.catch((err: any) => {
			console.log(err);
		});
};
