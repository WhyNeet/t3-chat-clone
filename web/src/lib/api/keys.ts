import { BACKEND_URI } from "../constants";
import type { ApiKey } from "../model/key";

export async function enrollKey(key: string, provider: string): Promise<string> {
  const response = await fetch(`${BACKEND_URI}/keys/enroll`, {
    credentials: "include",
    method: "POST",
    body: JSON.stringify({ key, provider }),
    headers: {
      "Content-Type": "application/json"
    },
  });
  const { id } = await response.json();

  return id;
}

export async function listKeys(): Promise<ApiKey[]> {
  const response = await fetch(`${BACKEND_URI}/keys`, {
    credentials: "include",
  });

  return await response.json();
}

export async function deleteKey(id: string): Promise<void> {
  await fetch(`${BACKEND_URI}/keys/${id}`, {
    credentials: "include",
    method: "DELETE"
  });
}
