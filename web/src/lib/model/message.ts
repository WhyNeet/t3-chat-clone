export interface ChatMessage {
  id: string;
  content: ChatMessageContent[];
  model: string | null;
  reasoning: string | null;
  role: Role;
  chat_id: string;
  timestamp: string;
}

export type ChatMessageContent = ChatMessageTextContent | ChatMessageImageContent | ChatMessagePdfContent;

export interface ChatMessageTextContent {
  type: "Text",
  value: string
}

export interface ChatMessageImageContent {
  type: "Image",
  id: string
}

export interface ChatMessagePdfContent {
  type: "Pdf",
  id: string
}

export enum Role {
  User = "User",
  Assistant = "Assistant"
}
