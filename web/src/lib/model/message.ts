export interface ChatMessage {
  id: string;
  content: string;
  role: Role;
  chat_id: string;
  timestamp: string;
}

export enum Role {
  User = "User",
  Assistant = "Assistant"
}
