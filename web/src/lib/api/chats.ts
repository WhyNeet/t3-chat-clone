import { BACKEND_URI } from "../constants";
import type { Chat } from "../model/chat";
import type { ListWindow } from "../util";

export async function chats(window: ListWindow): Promise<Chat[]> {
  const response = await fetch(`${BACKEND_URI}/chats?start=${Math.floor(window.start)}&take=${Math.floor(window.take)}`, {
    credentials: "include"
  });
  const chats = await response.json();

  return chats;
}
