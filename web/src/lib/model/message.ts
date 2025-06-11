export interface ChatMessage {
  id: string;
  content: string;
  reasoning: string | null;
  role: Role;
  chat_id: string;
  timestamp: string;
}

export enum Role {
  User = "User",
  Assistant = "Assistant"
}
