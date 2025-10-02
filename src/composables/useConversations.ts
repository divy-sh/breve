import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import type { Conversation, ConversationSummary } from '../types';

export function useConversations() {
  const conversations = ref<ConversationSummary[]>([]);
  const currentConversation = ref<Conversation | null>(null);
  const isDownloadingModel = ref<boolean>(false);

  /**
   * Load all conversations from the backend
   */
  async function loadConversations() {
    try {
      // Trigger model download in background if necessary. This returns immediately.
      // Fire-and-forget the ensure_model invoke so the UI doesn't wait on it.
      // Any errors are non-fatal and will be logged.
      invoke("ensure_model", {}).catch((e) => console.warn("ensure_model invoke failed:", e));

      // Listen for backend download events once (safe to call multiple times; listener will be a no-op if already set)
      // We don't await the listen promise here; update the reactive ref when events arrive.
      listen('downloading-model', (event) => {
        try {
          // payload should be a boolean
          isDownloadingModel.value = !!(event.payload as any);
        } catch (err) {
          console.warn('downloading-model event payload unexpected', err);
        }
      }).catch((e) => console.warn('failed to attach downloading-model listener', e));

      const ids = await invoke("get_conversation_ids") as string[];
      const loadedConversations: ConversationSummary[] = [];
      
      for (const id of ids) {
        const convo = await invoke("get_conversation", { convId: id });
        if (convo) {
          loadedConversations.push({
            id,
            title: (convo as Conversation).title || `Conversation ${id.substring(0, 8)}`
          });
        }
      }
      
      conversations.value = loadedConversations;
    } catch (error) {
      console.error("Error loading conversations:", error);
    }
  }

  /**
   * Load a specific conversation by ID
   */
  async function loadConversation(id: string) {
    try {
      const convo = await invoke("get_conversation", { convId: id });
      if (convo) {
        currentConversation.value = convo as Conversation;
      }
    } catch (error) {
      console.error("Error loading conversation:", error);
    }
  }

  /**
   * Start a new conversation with the given message
   */
  async function startNewConversation(message: string) {
    try {
      const convId = await invoke("start_conversation", {
        title: message.substring(0, 30),
      }) as string;
      
      // Load the new conversation after creation
      await loadConversation(convId);
      await loadConversations();
    } catch (error) {
      console.error("Error starting conversation:", error);
      throw error;
    }
  }

  /**
   * Continue an existing conversation with a new message
   */
  async function continueConversation(conversationId: string, message: string) {
    try {
      // Add user message to UI immediately for better responsiveness
      if (currentConversation.value && currentConversation.value.id === conversationId) {
        currentConversation.value.body.push({
          role: 'user',
          content: message
        });
      }
      
      // Send message to backend
      await invoke("continue_conversation", {
        convId: conversationId,
        userInput: message,
      });
      
      // Wait for streaming to complete and refresh conversation
      await loadConversation(conversationId);
    } catch (error) {
      console.error("Error continuing conversation:", error);
      throw error;
    }
  }

  async function deleteConversation(conversationId: string) {
    try {
      await invoke("delete_conversation", {
        convId: conversationId,
      }) as string;
      if (conversationId === currentConversation.value?.id) {
        currentConversation.value = null;
      }
      await loadConversations();
    } catch (error) {
      console.error("Error deleting conversation:", error);
      throw error;
    }
  }

  return {
    conversations,
    currentConversation,
    isDownloadingModel,
    loadConversations,
    loadConversation,
    startNewConversation,
    continueConversation,
    deleteConversation
  };
}