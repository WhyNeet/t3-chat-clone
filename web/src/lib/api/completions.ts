import { BACKEND_URI } from "../constants";
import type { Memory } from "../model/memory";
import type { ChatMessage } from "../model/message";

export function subscribeToStream(
  streamId: string,
  callback: (delta: OpenAICompletionDelta) => void,
  control: (control: CompletionControlData) => void,
  close?: (message: ChatMessage) => void,
) {
  const events = new EventSource(
    `${BACKEND_URI}/completions/prompt/sse/${streamId}`,
  );

  let retries = 0;

  events.addEventListener("error", () => {
    retries += 1;
    if (retries > 3) events.close();
  });

  events.addEventListener("message", (message: MessageEvent<string>) => {
    const data: CompletionData = JSON.parse(message.data);
    if (is.control(data)) {
      const controlData = data.control;
      if (is.done(controlData)) {
        events.close();
        close?.(controlData.message);
      } else {
        control(data);
      }

      return;
    }
    callback(data);
  });
}

export interface OpenAICompletionDelta {
  content: string;
  reasoning: string | null;
  role: string;
}

export enum ControlKind {
  Done = "Done",
  WebSearchPerformed = "WebSearchPerformed",
  ChatNameUpdated = "ChatNameUpdated",
  InferenceError = "InferenceError",
  MemoryAdded = "MemoryAdded"
}

export interface ControlDone {
  kind: ControlKind.Done;
  message: ChatMessage;
}

export interface ControlWebSearchPerformed {
  kind: ControlKind.WebSearchPerformed;
}

export interface ControlChatNameUpdated {
  kind: ControlKind.ChatNameUpdated;
  name: string;
}

export interface ControlMemoryAdded {
  kind: ControlKind.MemoryAdded;
  memory: Memory;
}

export interface ControlInferenceError {
  kind: ControlKind.InferenceError;
  code: number;
}

export interface CompletionControlData {
  control: ControlDone | ControlWebSearchPerformed | ControlChatNameUpdated | ControlInferenceError | ControlMemoryAdded;
}

export type CompletionData = OpenAICompletionDelta | CompletionControlData;

export const is = {
  control(data: CompletionData): data is CompletionControlData {
    return (
      (data as unknown as Record<string, string>)["control"] !== null &&
      (data as unknown as Record<string, string>)["control"] !== undefined
    );
  },
  done(data: CompletionControlData["control"]): data is ControlDone {
    return data.kind === ControlKind.Done;
  },
  webSearchPerformed(
    data: CompletionControlData["control"],
  ): data is ControlWebSearchPerformed {
    return data.kind === ControlKind.WebSearchPerformed;
  },
  chatNameUpdated(
    data: CompletionControlData["control"],
  ): data is ControlChatNameUpdated {
    return data.kind === ControlKind.ChatNameUpdated;
  },
  inferenceError(
    data: CompletionControlData["control"],
  ): data is ControlInferenceError {
    return data.kind === ControlKind.InferenceError;
  },
  memoryAdded(
    data: CompletionControlData["control"],
  ): data is ControlMemoryAdded {
    return data.kind === ControlKind.MemoryAdded;
  },
};
