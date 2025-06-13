import { BACKEND_URI } from "../constants";
import type { Chat } from "../model/chat";
import type { ListWindow } from "../util";

export async function chats(window: ListWindow): Promise<Chat[]> {
  const response = await fetch(
    `${BACKEND_URI}/chats?start=${Math.floor(window.start)}&take=${Math.floor(window.take)}`,
    {
      credentials: "include",
    },
  );
  const chats = await response.json();

  return chats;
}

export async function createChat(): Promise<Chat> {
  const response = await fetch(`${BACKEND_URI}/chats`, {
    credentials: "include",
    method: "POST",
  });
  const chat = await response.json();

  return chat;
}

export async function deleteChat(id: string): Promise<void> {
  await fetch(`${BACKEND_URI}/chats/${id}`, {
    credentials: "include",
    method: "DELETE",
  });
}

export async function renameChat(id: string, name: string): Promise<void> {
  await fetch(`${BACKEND_URI}/chats/${id}/rename`, {
    credentials: "include",
    method: "POST",
    body: JSON.stringify(name),
    headers: {
      "Content-Type": "application/json",
    },
  });
}
