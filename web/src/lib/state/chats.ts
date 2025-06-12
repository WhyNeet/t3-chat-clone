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
    { content: string; reasoning: string | null } | null
  >; // id -> pending message string
  isFetching: boolean;
  finishFetching: () => void;
  addChatMessages: (id: string, messages: ChatMessage[]) => void;
  updatePendingMessage: (
    id: string,
    delta: { content: string | null; reasoning: string | null },
  ) => void;
  setChatMessages: (id: string, messages: ChatMessage[]) => void;
  setChatState: (id: string, state: ChatState) => void;
  clearPendingMessage: (id: string) => void;
  initializeChat: (chat: Chat) => void;
}

export const useChatsStore = create<ChatStore>((set) => ({
  chats: {},
  pendingMessages: {},
  isFetching: true,
  finishFetching: () => set({ isFetching: false }),
  initializeChat: (chat) => {
    set((state) => ({
      chats: {
        ...state.chats,
        [chat.id]: {
          chat,
          state: { status: "idle" },
          messages: [],
          streaming: false,
        },
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
              ...(currentChatState.state.status === "success"
                ? currentChatState.messages
                : []),
            ],
          },
        },
      };
    });
  },
  updatePendingMessage: (id, delta) => {
    set((state) => {
      let reasoning = delta.content
        ? (state.pendingMessages[id]?.reasoning ?? null)
        : (state.pendingMessages[id]?.reasoning ?? "");
      if (reasoning) {
        if (delta.reasoning) reasoning += delta.reasoning;
      } else {
        if (delta.reasoning) reasoning += delta.reasoning;
      }
      return {
        pendingMessages: {
          ...state.pendingMessages,
          [id]: {
            content:
              (state.pendingMessages[id]?.content ?? "") +
              (delta.content ?? ""),
            reasoning,
          },
        },
        chats: {
          ...state.chats,
          [id]: {
            ...state.chats[id],
            streaming: true,
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
