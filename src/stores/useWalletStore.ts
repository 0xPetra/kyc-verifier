import { create } from "zustand";
import { devtools } from "zustand/middleware";
import { create as mutativeCreate, Draft } from 'mutative';

export interface WalletStore {
  ethAddress: string;
  eraPk: string;
  setEthAddress: (address: string) => void;
  setEraPk: (eraPk: string) => void;

}

export const mutative = (config) =>
  (set, get) => config((fn) => set(mutativeCreate(fn)), get);

type StoreSet = (fn: (draft: Draft<WalletStore>) => void) => void;

export const store = (set: StoreSet) => ({
  ethAddress: null,
  eraPk: null,
  setEthAddress: (address: string) => set((state) => { state.ethAddress = address }),
  setEraPk: (address: string) => set((state) => { state.eraPk = address }),
});

export const useWalletStore = create<WalletStore>()(devtools(mutative(store)));

