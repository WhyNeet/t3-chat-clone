// import type { ChatMessage } from "../model/message";
// import { createWithEqualityFn } from "zustand/traditional";

// export interface MessagesStore {
//   mapping: Map<string, ChatMessage[] | null>;
//   updateChatMessages: (id: string, messages: ChatMessage[]) => void;
//   addChatMessages: (id: string, messages: ChatMessage[]) => void;
// }

// export enum MessagesState {
//   Loading = 0,
//   Loaded = 1
// }

// export const useMessagesStore = createWithEqualityFn<MessagesStore>((set, get) => ({
//   mapping: new Map(),
//   addChatMessages: (id, messages) => {
//     const { mapping } = get();
//     mapping.set(id, [...(mapping.get(id) ?? []), ...messages]);
//     set({ mapping });
//   },
//   updateChatMessages: (id, messages) => {
//     const { mapping } = get();
//     mapping.set(id, messages);
//     set({ mapping });
//   },
// }), (prev, curr) => false);

// export interface PendingMessagesStore {
//   pendingMessages: Map<string, string>;
//   updatePendingMessage: (id: string, delta: string) => void;
// }

// export const usePendingMessagesStore = createWithEqualityFn<PendingMessagesStore>((set, get) => ({
//   pendingMessages: new Map(),
//   updatePendingMessage: (id, delta) => {
//     const { pendingMessages } = get();
//     pendingMessages.set(id, (pendingMessages.get(id) ?? "") + delta);
//     set({ pendingMessages });
//   }
// }), (prev, curr) => {
//   return false;
// })
