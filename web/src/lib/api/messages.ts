import { BACKEND_URI } from "../constants";
import type { ChatMessage } from "../model/message";
import type { ListWindow } from "../util";
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
}

export async function createMessage(chatId: string, message: SendMessagePayload): Promise<{ stream_id: string, user_message: ChatMessage }> {
  const response = await fetch(
    `${BACKEND_URI}/chats/${chatId}/message`,
    {
      credentials: "include",
      method: "POST",
      body: JSON.stringify(message),
      headers: {
        "Content-Type": "application/json"
      }
    },
  );
  const streamId = await response.json();

  return streamId;
}
