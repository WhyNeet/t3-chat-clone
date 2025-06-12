export interface ChatMessage {
  id: string;
  content: string;
  model: string | null;
  reasoning: string | null;
  role: Role;
  chat_id: string;
  timestamp: string;
}

export enum Role {
  User = "User",
  Assistant = "Assistant"
}
