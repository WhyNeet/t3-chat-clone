import { create } from "zustand";
import type { Chat } from "../model/chat";
import type { ChatMessage } from "../model/message";
import type { UserUpload } from "../model/upload";

export type ChatState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success" }
  | { status: "error"; error: string };

export interface ChatStore {
  chats: Record<
    string,
    {
      chat: Chat;
      state: ChatState;
      messages: ChatMessage[];
      streaming: boolean;
      searching: boolean;
    }
  >; // id -> ChatState
  pendingMessages: Record<
    string,
    {
      content: string;
      reasoning: string | null;
      model: string;
      search: boolean;
      memory: string | null;
    } | null
  >; // id -> pending message string
  uploads: Record<string, UserUpload[]>;
  addUpload: (id: string, upload: UserUpload) => void;
  removeUpload: (id: string, fileId: string) => void;
  clearUploads: (id: string) => void;
  setUploads: (id: string, uploads: UserUpload[]) => void;
  isFetching: boolean;
  deleteChat: (id: string) => void;
  renameChat: (id: string, name: string) => void;
  finishFetching: () => void;
  finishWebSearch: (id: string) => void;
  addChatMessages: (id: string, messages: ChatMessage[]) => void;
  prependChatMessages: (id: string, messages: ChatMessage[]) => void;
  updateChatName: (id: string, name: string) => void;
  initPendingMessage: (id: string, model: string, search: boolean, memory: string | null) => void;
  updatePendingMessage: (
    id: string,
    delta: { content: string | null; reasoning: string | null, memory: string | null },
  ) => void;
  setChatMessages: (id: string, messages: ChatMessage[]) => void;
  setChatState: (id: string, state: ChatState) => void;
  clearPendingMessage: (id: string) => void;
  updatePendingMessageMemory: (id: string, memory: string) => void;
  initializeChat: (chat: Chat, messages?: ChatMessage[]) => void;
}

export const useChatsStore = create<ChatStore>((set, get) => ({
  chats: {},
  pendingMessages: {},
  uploads: {},
  isFetching: true,
  clearUploads: (id) => set(state => ({ uploads: { ...state.uploads, [id]: [] } })),
  addUpload: (id, upload) =>
    set((state) => ({
      uploads: {
        ...state.uploads,
        [id]: [...(state.uploads[id] ?? []), upload],
      },
    })),
  removeUpload: (id, fileId) =>
    set((state) => ({
      uploads: {
        ...state.uploads,
        [id]: (state.uploads[id] ?? []).filter((file) => file.id !== fileId),
      },
    })),
  setUploads: (id, uploads) =>
    set((state) => ({ uploads: { ...state.uploads, [id]: uploads } })),
  renameChat: (id, name) => {
    console.log(get().chats, id);
    set((state) => ({
      chats: {
        ...state.chats,
        [id]: { ...state.chats[id], chat: { ...state.chats[id].chat, name } },
      },
    }));
  },
  deleteChat: (id) => {
    if (get().pendingMessages[id]?.content !== undefined) return;
    const chats = { ...get().chats };
    delete chats[id];
    set({ chats });
  },
  finishInitialFetch: () => set({ chats: {} }),
  updateChatName: (id, name) =>
    set((state) => ({
      ...state,
      chats: {
        ...state.chats,
        [id]: { ...state.chats[id], chat: { ...state.chats[id].chat, name } },
      },
    })),
  finishFetching: () => set({ isFetching: false }),
  initializeChat: (chat, messages) => {
    set((state) => ({
      chats: {
        [chat.id]: {
          chat,
          state: { status: messages ? "success" : "idle" },
          messages: messages ?? [],
          streaming: false,
          searching: false,
        },
        ...state.chats,
      },
      pendingMessages: { ...state.pendingMessages, [chat.id]: null },
    }));
  },
  finishWebSearch: (id) => {
    set((state) => ({
      ...state,
      chats: { ...state.chats, [id]: { ...state.chats[id], searching: false } },
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
  prependChatMessages: (id, messages) => {
    set((state) => ({
      chats: {
        ...state.chats,
        [id]: {
          ...state.chats[id]!,
          messages: [
            ...state.chats[id].messages,
            ...(messages.length > 0 &&
              messages[0].id === state.chats[id].messages[0].id
              ? []
              : messages),
          ],
        },
      },
    }));
  },
  addChatMessages: (id: string, messages: ChatMessage[]) => {
    set((state) => {
      const currentChatState = state.chats[id];
      return {
        chats: {
          ...state.chats,
          [id]: {
            ...currentChatState,
            messages: [...messages, ...currentChatState.messages],
          },
        },
      };
    });
  },
  initPendingMessage: (id, model, search, memory) => {
    set((state) => ({
      pendingMessages: {
        ...state.pendingMessages,
        [id]: {
          model,
          content: "",
          reasoning: null,
          search,
          memory
        },
      },
      chats: {
        ...state.chats,
        [id]: {
          ...state.chats[id],
          streaming: true,
          searching: search,
        },
      },
    }));
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
            content: state.pendingMessages[id]!.content + (delta.content ?? ""),
            reasoning,
            memory: state.pendingMessages[id]!.memory ?? delta.memory
          },
        },
      };
    });
  },
  updatePendingMessageMemory: (id, memory) => {
    set(state => ({ pendingMessages: { ...state.pendingMessages, [id]: { ...state.pendingMessages[id]!, memory } } }))
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
