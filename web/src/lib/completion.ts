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
