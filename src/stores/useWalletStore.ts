import { create } from "zustand";
import { devtools } from "zustand/middleware";
import { create as mutativeCreate, Draft } from 'mutative';

export interface WalletStore {
  ethAddress: string;
  eraPk: string;
  setEthAddress: (address: string) => void;
  setEraPk: (address: string) => void;

}

export const mutative = (config) =>
  (set, get) => config((fn) => set(mutativeCreate(fn)), get);

type StoreSet = (fn: (draft: Draft<WalletStore>) => void) => void;

const store = (set: StoreSet) => ({
  ethAddress: null,
  setEthAddress: (address) => set(() => ({ ethAddress: address })),
  eraPk: null,
  setEraPk: (address) => set(() => ({ eraPk: address })),

});
export const useWalletStore = create<WalletStore>()(devtools(mutative(store)));
