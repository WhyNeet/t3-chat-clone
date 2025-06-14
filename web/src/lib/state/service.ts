import { create } from "zustand";
import type { Model } from "../model/service";
import type { ApiKey } from "../model/key";

export interface ServiceStore {
  models: { free: Model[], paid: Model[] } | null;
  keys: ApiKey[] | null;
  setKeys: (keys: ApiKey[]) => void;
  removeKey: (id: string) => void;
  addKey: (key: ApiKey) => void;
  inferenceError: Record<string, number>; // Provier name -> error
  setModels: (models: { free: Model[], paid: Model[] }) => void;
  setInferenceError: (provider: string, code: number) => void;
}

export const useServiceStore = create<ServiceStore>((set) => ({
  models: null,
  keys: null,
  inferenceError: {},
  setKeys: (keys) => set({ keys }),
  addKey: (key) => set(state => ({ keys: state.keys ? [...state.keys, key] : null })),
  removeKey: (id) => {
    set((state) => ({
      keys: (state.keys ?? []).filter((k) => k.id !== id),
    }));
  },
  setModels: (models) => set({ models }),
  setInferenceError: (provider, error) =>
    set((state) => ({
      inferenceError: { ...state.inferenceError, [provider]: error },
    })),
}));
