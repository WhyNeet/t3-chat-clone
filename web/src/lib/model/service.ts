export interface Model {
  name: string;
  identifier: string;
  api_kind: ModelApiKind;
  base_url: string;
  is_reasoning: string;
  author: string;
}

export enum ModelApiKind {
  OpenAI = "openai",
}
