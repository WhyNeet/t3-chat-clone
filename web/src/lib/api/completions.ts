import { BACKEND_URI } from "../constants";

export function subscribeToStream(streamId: string, callback: (delta: OpenAICompletionDelta) => void, close?: () => void) {
  const events = new EventSource(
    `${BACKEND_URI}/completions/prompt/sse/${streamId}`,
  );

  events.addEventListener("message", (message: MessageEvent<string>) => {
    const data: CompletionData = JSON.parse(message.data);
    if (isControl(data)) {
      switch (data.control) {
        case "done":
          events.close();
          close?.();
          return;
      }
    }
    callback(data);
  });
}

export interface OpenAICompletionDelta {
  content: string;
  role: string;
}

export interface CompletionControlData {
  control: "done";
}

export type CompletionData = OpenAICompletionDelta | CompletionControlData;

export function isControl(data: CompletionData): data is CompletionControlData {
  return (
    (data as unknown as Record<string, string>)["control"] !== null &&
    (data as unknown as Record<string, string>)["control"] !== undefined
  );
}
