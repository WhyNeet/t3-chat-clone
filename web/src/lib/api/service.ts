import { BACKEND_URI } from "../constants";
import type { Model } from "../model/service";

export async function listModels(): Promise<Model[]> {
  const response = await fetch(`${BACKEND_URI}/models`);
  const models = await response.json();

  return models;
}
