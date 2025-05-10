export interface Message {
  role: 'user' | 'assistant';
  content: string;
}

export interface Conversation {
  id: string;
  title: string;
  messages: Message[];
  createdAt?: string;  // Optional timestamp
}

export interface ConversationSummary {
  id: string;
  title: string;
}