import { create } from "zustand";
import type { ChatMessage } from "../model/message";

export interface MessagesStore {
  mapping: Map<string, ChatMessage[] | null>;
  updateChatMessages: (id: string, messages: ChatMessage[]) => void;
  addChatMessages: (id: string, messages: ChatMessage[]) => void;
}

export enum MessagesState {
  Loading = 0,
  Loaded = 1
}

export const useMessagesStore = create<MessagesStore>((set, get) => ({
  mapping: new Map(),
  addChatMessages: (id, messages) => {
    const { mapping } = get();
    mapping.set(id, [...(mapping.get(id) ?? []), ...messages]);
    set({ mapping });
  },
  updateChatMessages: (id, messages) => {
    const { mapping } = get();
    mapping.set(id, messages);
    set({ mapping });
  }
}));
