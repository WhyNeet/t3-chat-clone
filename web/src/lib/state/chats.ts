import { create } from "zustand";
import type { Chat } from "../model/chat";
import type { ChatMessage } from "../model/message";

// export interface ChatsStore {
//   chats: Chat[] | null;
//   state: ChatsState;
//   error: string | null;
//   updateChats: (chats: Chat[]) => void;
//   addChat: (chat: Chat) => void;
//   updateError: (error: string) => void;
// }

// export enum ChatsState {
//   Loading = 0,
//   Loaded = 1,
// }

// export const useChatsStore = create<ChatsStore>((set) => ({
//   chats: null,
//   state: ChatsState.Loading,
//   error: null,
//   updateChats: (chats) => set({ chats, state: ChatsState.Loaded }),
//   addChat: (chat) => set(({ chats }) => ({ chats: [...(chats ?? []), chat] })),
//   updateError: (error) => set({ error }),
// }));

export type ChatState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success" }
  | { status: "error"; error: string };

export interface ChatStore {
  chats: Record<
    string,
    { chat: Chat; state: ChatState; messages: ChatMessage[]; streaming: boolean }
  >; // id -> ChatState
  pendingMessages: Record<
    string,
    { content: string; reasoning: string | null, model: string } | null
  >; // id -> pending message string
  isFetching: boolean;
  finishFetching: () => void;
  addChatMessages: (id: string, messages: ChatMessage[]) => void;
  initPendingMessage: (
    id: string,
    model: string
  ) => void;
  updatePendingMessage: (
    id: string,
    delta: { content: string | null; reasoning: string | null },
  ) => void;
  setChatMessages: (id: string, messages: ChatMessage[]) => void;
  setChatState: (id: string, state: ChatState) => void;
  clearPendingMessage: (id: string) => void;
  initializeChat: (chat: Chat, messages?: ChatMessage[]) => void;
}

export const useChatsStore = create<ChatStore>((set) => ({
  chats: {},
  pendingMessages: {},
  isFetching: true,
  finishFetching: () => set({ isFetching: false }),
  initializeChat: (chat, messages) => {
    set((state) => ({
      chats: {
        [chat.id]: {
          chat,
          state: { status: messages ? "success" : "idle" },
          messages: messages ?? [],
          streaming: false,
        },
        ...state.chats,
      },
      pendingMessages: { ...state.pendingMessages, [chat.id]: null },
    }));
  },
  setChatMessages: (id, messages) => {
    set((state) => {
      const currentChatState = state.chats[id];
      return {
        chats: {
          ...state.chats,
          [id]: {
            ...currentChatState,
            messages,
          },
        },
      };
    });
  },
  addChatMessages: (id: string, messages: ChatMessage[]) => {
    set((state) => {
      const currentChatState = state.chats[id];
      return {
        chats: {
          ...state.chats,
          [id]: {
            ...currentChatState,
            messages: [
              ...messages,
              ...currentChatState.messages,
            ],
          },
        },
      };
    });
  },
  initPendingMessage: (id, model) => {
    set(state => ({
      pendingMessages: {
        ...state.pendingMessages,
        [id]: {
          model,
          content: "",
          reasoning: null,
        },
      },
      chats: {
        ...state.chats,
        [id]: {
          ...state.chats[id],
          streaming: true,
        },
      },
    }))
  },
  updatePendingMessage: (id, delta) => {
    set((state) => {
      let reasoning = delta.reasoning
        ? (state.pendingMessages[id]!.reasoning ?? "")
        : (state.pendingMessages[id]!.reasoning ?? null);
      if (delta.reasoning) reasoning += delta.reasoning;
      return {
        pendingMessages: {
          ...state.pendingMessages,
          [id]: {
            ...state.pendingMessages[id]!,
            content:
              state.pendingMessages[id]!.content +
              (delta.content ?? ""),
            reasoning,
          },
        },
      };
    });
  },
  clearPendingMessage: (id: string) => {
    set((state) => ({
      pendingMessages: {
        ...state.pendingMessages,
        [id]: null,
      },
      chats: {
        ...state.chats,
        [id]: {
          ...state.chats[id],
          streaming: false,
        },
      },
    }));
  },
  setChatState: (id: string, state: ChatState) => {
    set((s) => ({ chats: { ...s.chats, [id]: { ...s.chats[id], state } } }));
  },
}));
