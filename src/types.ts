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

export interface StreamPayload {
  id: string;
  content: string;
}

// TODO add other fields as required
export interface ModelConfig {
  temperature: number;
  system_prompt: string;
  max_output_length: number;
  max_context_length: number;
}