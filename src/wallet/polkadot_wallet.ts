import { web3Enable, web3Accounts, web3FromAddress } from '@polkadot/extension-dapp';

export interface WalletAccount {
    address: string;
    name: string | null;
    source: string;
}

// Enable wallet extensions and get available accounts
export async function connectWallet(appName: string): Promise<WalletAccount[]> {
    try {
        // This prompts the user to authorize your dapp
        const extensions = await web3Enable(appName);
        
        if (extensions.length === 0) {
            throw new Error('No wallet extension found. Please install Polkadot.js, Talisman, or SubWallet.');
        }
        
        // Get all accounts from all extensions
        const allAccounts = await web3Accounts();
        
        return allAccounts.map(account => ({
            address: account.address,
            name: account.meta.name || null,
            source: account.meta.source,
        }));
    } catch (error) {
        console.error('Failed to connect wallet:', error);
        throw error;
    }
}

// Check if any wallet extension is available
export async function isWalletAvailable(): Promise<boolean> {
    // Check if we're in a browser environment
    if (typeof window === 'undefined') {
        return false;
    }
    
    // Check for injected wallet
    return !!(window as any).injectedWeb3;
}

// Get signer for a specific address (for signing transactions)
export async function getWalletSigner(address: string) {
    const injector = await web3FromAddress(address);
    return injector.signer;
}