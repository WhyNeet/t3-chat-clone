import { BACKEND_URI } from "../constants";
import type { ChatMessage } from "../model/message";
import type { ListWindow } from "../util";
import type { ApiError } from "./error";

export async function fetchChatMessages(
  chat_id: string,
  window: ListWindow,
): Promise<ChatMessage[] | ApiError> {
  const response = await fetch(
    `${BACKEND_URI}/chats/${chat_id}/messages?start=${Math.floor(window.start)}&take=${Math.floor(window.take)}`,
    {
      credentials: "include",
    },
  );
  const messages = await response.json().catch(() => null);
  if (!response.ok && !messages) return { error: "Invalid Chat ID." };

  return messages;
}
