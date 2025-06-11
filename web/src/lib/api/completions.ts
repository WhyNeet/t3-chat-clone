import { BACKEND_URI } from "../constants";
import type { ChatMessage } from "../model/message";

export function subscribeToStream(streamId: string, callback: (delta: OpenAICompletionDelta) => void, close?: (message: ChatMessage) => void) {
  const events = new EventSource(
    `${BACKEND_URI}/completions/prompt/sse/${streamId}`,
  );

  events.addEventListener("message", (message: MessageEvent<string>) => {
    const data: CompletionData = JSON.parse(message.data);
    if (isControl(data)) {
      switch (data.control) {
        case "done":
          events.close();
          close?.(data.message);
          return;
      }
    }
    callback(data);
  });
}

export interface OpenAICompletionDelta {
  content: string;
  reasoning: string | null;
  role: string;
}

export interface CompletionControlData {
  control: "done";
  message: ChatMessage;
}

export type CompletionData = OpenAICompletionDelta | CompletionControlData;

export function isControl(data: CompletionData): data is CompletionControlData {
  return (
    (data as unknown as Record<string, string>)["control"] !== null &&
    (data as unknown as Record<string, string>)["control"] !== undefined
  );
}
