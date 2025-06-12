import { create } from "zustand";
import type { Model } from "../model/service";

export interface ServiceStore {
  models: Model[] | null;
  setModels: (models: Model[]) => void;
}

export const useServiceStore = create<ServiceStore>((set) => ({
  models: null,
  setModels: (models) => set({ models }),
}));
