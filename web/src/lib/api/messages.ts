import { BACKEND_URI } from "../constants";
import type { Memory } from "../model/memory";
import type { ChatMessage } from "../model/message";
import type { Model } from "../model/service";
import { useChatsStore } from "../state/chats";
import { useMemoryStore } from "../state/memory";
import { useServiceStore } from "../state/service";
import type { ListWindow } from "../util";
import { is, subscribeToStream } from "./completions";
import type { ApiError } from "./error";

export async function fetchChatMessages(
  chatId: string,
  window: ListWindow,
  share_id?: string
): Promise<ChatMessage[] | ApiError> {
  const response = await fetch(
    `${BACKEND_URI}/chats/${chatId}/messages?start=${Math.floor(window.start)}&take=${Math.floor(window.take)}${share_id ? `&share_id=${share_id}` : ""}`,
    {
      credentials: "include",
    },
  );
  const messages = await response.json().catch(() => null);
  if (!response.ok && !messages) return { error: "Invalid Chat ID." };

  return messages;
}

export interface SendMessagePayload {
  message: string;
  model: string;
  reasoning: "high" | "medium" | "low" | null;
  use_search: boolean;
  use_memories: boolean;
}

export async function createMessage(
  chatId: string,
  message: SendMessagePayload,
): Promise<{ stream_id: string; user_message: ChatMessage }> {
  const response = await fetch(`${BACKEND_URI}/chats/${chatId}/message`, {
    credentials: "include",
    method: "POST",
    body: JSON.stringify(message),
    headers: {
      "Content-Type": "application/json",
    },
  });
  const streamId = await response.json();

  return streamId;
}

export const sendAndSubscribe = async (
  chatId: string,
  model: Model,
  payload: SendMessagePayload,
  onRequestFinished: () => void,
) => {
  const {
    addChatMessages: addMessages,
    initPendingMessage,
    updatePendingMessage,
    finishWebSearch,
    updateChatName,
    clearPendingMessage,
    updatePendingMessageMemory,
    setChatState,
  } = useChatsStore.getState();
  const { setInferenceError, removeInferenceError } = useServiceStore.getState();
  const { addMemory } = useMemoryStore.getState();
  const provider = model.base_url;
  removeInferenceError("openrouter");
  const { stream_id, user_message } = await createMessage(chatId, payload);
  addMessages(chatId, [user_message]);
  localStorage.setItem(`chat-model-${chatId}`, model.identifier);
  localStorage.setItem(`stream-${chatId}`, stream_id);
  if (payload.use_search)
    localStorage.setItem(`streaming-message-${chatId}-search`, "");
  onRequestFinished();
  initPendingMessage(chatId, model.name, payload.use_search, null);
  subscribeToStream(
    stream_id,
    (delta) => {
      updatePendingMessage(chatId, {
        content: delta.content,
        reasoning: delta.reasoning,
        memory: null
      });
      localStorage.setItem(
        `streaming-message-${chatId}`,
        (localStorage.getItem(`streaming-message-${chatId}`) ?? "") +
        delta.content,
      );
      localStorage.setItem(
        `streaming-message-reasoning-${chatId}`,
        (localStorage.getItem(`streaming-message-reasoning-${chatId}`) ?? "") +
        (delta.reasoning ?? ""),
      );
      localStorage.setItem(`streaming-message-${chatId}-model`, model.name);
    },
    (control) => {
      if (is.webSearchPerformed(control.control)) {
        localStorage.removeItem(`streaming-message-${chatId}-search`);
        finishWebSearch(chatId);
      } else if (is.chatNameUpdated(control.control)) {
        updateChatName(chatId, control.control.name);
      } else if (is.inferenceError(control.control)) {
        console.log("error", provider, control);
        if (
          provider === "https://openrouter.ai/api/v1/chat/completions"
        ) {
          setInferenceError("openrouter", control.control.code);
          console.error("code", control.control.code);
        }
        localStorage.removeItem(`stream-${chatId}`);
        localStorage.removeItem(`streaming-message-${chatId}`);
        localStorage.removeItem(`streaming-message-reasoning-${chatId}`);
        localStorage.removeItem(`streaming-message-${chatId}-search`);
        localStorage.removeItem(`streaming-message-${chatId}-model`);
        localStorage.removeItem(
          `streaming-message-${chatId}-memory`
        );
        clearPendingMessage(chatId);
      } else if (is.memoryAdded(control.control)) {
        localStorage.setItem(
          `streaming-message-${chatId}-memory`,
          JSON.stringify(control.control.memory),
        );
        updatePendingMessageMemory(chatId, control.control.memory.content);
        addMemory(control.control.memory);
      }
    },
    (message) => {
      // memory is not sent with "done" chunk
      const memoryString = localStorage.getItem(
        `streaming-message-${chatId}-memory`
      );
      const memory: Memory | null = memoryString ? JSON.parse(memoryString) : null;

      localStorage.removeItem(`stream-${chatId}`);
      localStorage.removeItem(`streaming-message-${chatId}`);
      localStorage.removeItem(`streaming-message-reasoning-${chatId}`);
      localStorage.removeItem(`streaming-message-${chatId}-search`);
      localStorage.removeItem(`streaming-message-${chatId}-model`);
      localStorage.removeItem(
        `streaming-message-${chatId}-memory`
      );
      clearPendingMessage(chatId);
      addMessages(chatId, [{ ...message, updated_memory: memory?.content ?? null }]);
      setChatState(chatId, { status: "success" });
    },
  );
};
