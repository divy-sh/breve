export interface Message {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export interface Conversation {
  id: string;
  title: string;
  body: Message[];
}

export interface ConversationSummary {
  id: string;
  title: string;
}