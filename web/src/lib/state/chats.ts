import { create } from "zustand";
import type { Chat } from "../model/chat";

export interface ChatsStore {
  chats: Chat[] | null;
  state: ChatsState;
  error: string | null;
  updateChats: (chats: Chat[]) => void;
  updateError: (error: string) => void;
}

export enum ChatsState {
  Loading = 0,
  Loaded = 1,
}

export const useChatsStore = create<ChatsStore>((set) => ({
  chats: null,
  state: ChatsState.Loading,
  error: null,
  updateChats: (chats) => set({ chats, state: ChatsState.Loaded }),
  updateError: (error) => set({ error })
}));
