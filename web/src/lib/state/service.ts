import { create } from "zustand";
import type { Model } from "../model/service";

export interface ServiceStore {
  models: Model[] | null;
  inferenceError: Record<string, number>; // Provier name -> error
  setModels: (models: Model[]) => void;
  setInferenceError: (provider: string, code: number) => void;
}

export const useServiceStore = create<ServiceStore>((set) => ({
  models: null,
  inferenceError: {},
  setModels: (models) => set({ models }),
  setInferenceError: (provider, error) =>
    set((state) => ({
      inferenceError: { ...state.inferenceError, [provider]: error },
    })),
}));
