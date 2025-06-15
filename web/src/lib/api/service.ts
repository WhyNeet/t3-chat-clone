import { BACKEND_URI } from "../constants";
import type { Memory } from "../model/memory";
import type { Model } from "../model/service";

export async function listModels(): Promise<{ free: Model[], paid: Model[] }> {
  const response = await fetch(`${BACKEND_URI}/models`);
  const models = await response.json();

  return models;
}

export async function listMemories(): Promise<Memory[]> {
  const response = await fetch(`${BACKEND_URI}/memories`, {
    credentials: "include"
  });
  const memories = await response.json();

  return memories;
}

export async function removeMemory(id: string): Promise<void> {
  await fetch(`${BACKEND_URI}/memories/${id}`, {
    credentials: "include",
    method: "DELETE"
  });
}