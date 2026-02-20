import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { Conversation, ConversationSummary } from '../types';
import { useSettings } from './useSettings';

const conversations = ref<ConversationSummary[]>([]);
const currentConversation = ref<Conversation | null>(null);
const modelStatus = ref<string>("UNSET");
const availableModels = ref<Record<string,string>>({});
const downloadedModels = ref<string[]>([]);
const defaultModel = ref<string>("");
const { setConfig } = useSettings();

export function useConversations() {

  /**
   * Load all conversations from the backend
   */
  async function loadConversations() {
    try {
      // Listen for backend download events once (safe to call multiple times; listener will be a no-op if already set)
      // We don't await the listen promise here; update the reactive ref when events arrive.
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
      await setConfig("lastConversationId", convId);
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

  async function downloadModel(modelName?: string) {
    try {
      await invoke("download_model", { modelName: modelName });
    } catch (error) {
      console.error("Error downloading model: ", error);
      throw error;
    }
  }

  async function getAvailableModels() {
    try {
      const res = await invoke("get_available_models") as Record<string, string>;
      return res;
    } catch (err) {
      console.error("Error fetching available models", err);
      return {};
    }
  }

  async function listDownloadedModels() {
    try {
      const res = await invoke("list_downloaded_models") as string[];
      return res.sort();
    } catch (err) {
      console.error("Error fetching downloaded models", err);
      return [] as string[];
    }
  }

  async function deleteModel(modelName: string) {
    try {
      await invoke("delete_model", { modelName });
    } catch (err) {
      console.error("Error deleting model", err);
      throw err;
    }
  }

  async function setDefaultModel(modelName: string) {
    try {
      await invoke("set_default_model", { modelName });
    } catch (err) {
      console.error("Error setting default model", err);
      throw err;
    }
  }

  async function getDefaultModel() {
    try {
      const res = await invoke("get_default_model") as string;
      return res;
    } catch (err) {
      console.error("Error fetching default model", err);
      return "";
    }
  }

    async function getModelStatus() {
    try {
      const res = await invoke("get_model_status") as string;
      return res;
    } catch (err) {
      console.error("Error fetching model status", err);
      return "";
    }
  }

  async function abortGeneration() {
    try {
      await invoke("abort_generation");
    } catch (err) {
      console.error("Error aborting generation", err);
    }
  }

  return {
    conversations,
    currentConversation,
    modelStatus,
    availableModels,
    downloadedModels,
    defaultModel,
    loadConversations,
    loadConversation,
    startNewConversation,
    continueConversation,
    deleteConversation,
    downloadModel,
    getAvailableModels,
    listDownloadedModels,
    deleteModel,
    setDefaultModel,
    getDefaultModel,
    getModelStatus,
    abortGeneration
  };
}