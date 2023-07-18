import { create } from "zustand";
import { devtools } from "zustand/middleware";
import { create as mutativeCreate, Draft } from 'mutative';

export interface WalletStore {
  ethAddress: string;
  eraWallet: string;
  setEthAddress: (address: string) => void;
  setWallet: (address: string) => void;

}

export const mutative = (config) =>
  (set, get) => config((fn) => set(mutativeCreate(fn)), get);

type StoreSet = (fn: (draft: Draft<WalletStore>) => void) => void;

export const store = (set: StoreSet) => ({
  ethAddress: null,
  eraWallet: null,
  setEthAddress: (address: string) => set((state) => { state.ethAddress = address }),
  setWallet: (address: string) => set((state) => { state.eraWallet = address }),
});

export const useWalletStore = create<WalletStore>()(devtools(mutative(store)));


