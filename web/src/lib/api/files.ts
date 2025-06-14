import { BACKEND_URI } from "../constants";
import type { UserUpload } from "../model/upload";

export async function uploadFile(
  chatId: string | null,
  formData: FormData,
): Promise<UserUpload> {
  const response = await fetch(`${BACKEND_URI}/files/${chatId ?? "nochat"}`, {
    credentials: "include",
    body: formData,
    method: "POST",
  });
  const upload = await response.json();

  return upload;
}

export async function listUnsentFiles(chatId: string | null): Promise<UserUpload[]> {
  const response = await fetch(`${BACKEND_URI}/files/${chatId ?? "nochat"}/unsent`, {
    credentials: "include",
  });
  const uploads = await response.json();

  return uploads;
}

export function getFileUri(chatId: string | null, fileId: string): string {
  return `${BACKEND_URI}/files/${chatId ?? "nochat"}/${fileId}`;
}

export async function deleteFile(chatId: string | null, fileId: string): Promise<void> {
  await fetch(`${BACKEND_URI}/files/${chatId ?? "nochat"}/${fileId}`, {
    credentials: "include",
    method: "DELETE"
  });
}
