import { create } from "zustand";
import type { User } from "../user";

export interface AuthStore {
  user: User | null,
  state: AuthState
  updateUser: (user: User) => void,
  clearUser: () => void
}

export enum AuthState {
  Loading = 0,
  Loaded = 1
}

export const useAuthStore = create<AuthStore>((set) => ({
  user: null,
  state: AuthState.Loading,
  updateUser: (user: User) => set({ user, state: AuthState.Loaded }),
  clearUser: () => set({ user: null })
}));
