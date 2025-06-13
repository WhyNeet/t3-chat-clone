import { BACKEND_URI } from "../constants";
import type { ChatMessage } from "../model/message";
import type { Model } from "../model/service";
import { useChatsStore } from "../state/chats";
import { useServiceStore } from "../state/service";
import type { ListWindow } from "../util";
import { is, subscribeToStream } from "./completions";
import type { ApiError } from "./error";

export async function fetchChatMessages(
  chatId: string,
  window: ListWindow,
): Promise<ChatMessage[] | ApiError> {
  const response = await fetch(
    `${BACKEND_URI}/chats/${chatId}/messages?start=${Math.floor(window.start)}&take=${Math.floor(window.take)}`,
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
    setChatState,
  } = useChatsStore.getState();
  const { setInferenceError } = useServiceStore.getState();
  const provider = model.base_url;
  const { stream_id, user_message } = await createMessage(chatId, payload);
  addMessages(chatId, [user_message]);
  localStorage.setItem(`chat-model-${chatId}`, model.identifier);
  localStorage.setItem(`stream-${chatId}`, stream_id);
  if (payload.use_search)
    localStorage.setItem(`streaming-message-${chatId}-search`, "");
  onRequestFinished();
  initPendingMessage(chatId, model.name, payload.use_search);
  subscribeToStream(
    stream_id,
    (delta) => {
      updatePendingMessage(chatId, {
        content: delta.content,
        reasoning: delta.reasoning,
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
        if (
          provider === "https://https://openrouter.ai/api/v1/chat/completions"
        ) {
          setInferenceError("openrouter", control.control.error);
        }
        localStorage.removeItem(`stream-${chatId}`);
        localStorage.removeItem(`streaming-message-${chatId}`);
        localStorage.removeItem(`streaming-message-reasoning-${chatId}`);
        localStorage.removeItem(`streaming-message-${chatId}-search`);
        localStorage.removeItem(`streaming-message-${chatId}-model`);
        clearPendingMessage(chatId);
      }
    },
    (message) => {
      localStorage.removeItem(`stream-${chatId}`);
      localStorage.removeItem(`streaming-message-${chatId}`);
      localStorage.removeItem(`streaming-message-reasoning-${chatId}`);
      localStorage.removeItem(`streaming-message-${chatId}-search`);
      localStorage.removeItem(`streaming-message-${chatId}-model`);
      clearPendingMessage(chatId);
      addMessages(chatId, [message]);
      setChatState(chatId, { status: "success" });
    },
  );
};
