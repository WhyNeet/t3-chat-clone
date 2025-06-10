import { create } from "zustand";
import type { User } from "../model/user";

export interface AuthStore {
  user: User | null;
  state: AuthState;
  error: string | null;
  updateUser: (user: User | null) => void;
  updateError: (error: string) => void;
  clearUser: () => void;
}

export enum AuthState {
  Loading = 0,
  Loaded = 1,
}

export const useAuthStore = create<AuthStore>((set) => ({
  user: null,
  state: AuthState.Loading,
  error: null,
  updateUser: (user) => set({ user, state: AuthState.Loaded }),
  updateError: (error) => set({ error }),
  clearUser: () => set({ user: null }),
}));
