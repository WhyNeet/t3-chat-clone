import { BACKEND_URI } from "../constants";
import type { Chat } from "../model/chat";
import type { Share } from "../model/share";
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

export async function getChatShate(id: string, share_id?: string): Promise<Chat | null> {
  const response = await fetch(`${BACKEND_URI}/chats/${id}${share_id ? `?share_id=${share_id}` : ""}`, {
    credentials: "include",
    method: "GET",
  });
  if (!response.ok) return null;
  const chat = await response.json();

  return chat;
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
    body: JSON.stringify({ name }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export async function shareChat(id: string): Promise<Share> {
  const response = await fetch(`${BACKEND_URI}/chats/${id}/share`, {
    credentials: "include",
    method: "POST",
  });
  const { id: share_id } = await response.json();

  return { share_id, id };
}

export async function getShareState(id: string): Promise<Share | null> {
  const response = await fetch(`${BACKEND_URI}/chats/${id}/share`, {
    credentials: "include",
    method: "GET",
  });
  if (!response.ok) return null;
  const share = await response.json();

  return share;
}

export async function unshareChat(id: string, share_id: string): Promise<void> {
  await fetch(`${BACKEND_URI}/chats/${id}/share/${share_id}`, {
    credentials: "include",
    method: "DELETE",
  });
}