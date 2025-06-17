export interface Model {
  name: string;
  identifier: string;
  provider: ModelProvider,
  base_url: string;
  is_reasoning: string;
  author: string;
}

export enum ModelProvider {
  OpenRouter = "OpenRouter",
  Chutes = "Chutes"
}
