<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from "vue";
import { listen } from "@tauri-apps/api/event";

import Sidebar from "./components/Sidebar.vue";
import ChatContainer from "./components/ChatContainer.vue";
import { useConversations } from "./composables/useConversations";

// Types

// State
const darkMode = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);
const sidebarOpen = ref(window.innerWidth > 768);

// Use composable for conversation management
const { 
  conversations, 
  currentConversation, 
  loadConversations, 
  loadConversation, 
  startNewConversation,
  continueConversation,
  deleteConversation
} = useConversations();

// Streaming state
const currentStreamingContent = ref("");
const isLoading = ref(false);

// Unlisten function for the event listener
let unlisten: (() => void) | null = null;

// Setup event listener for LLM streaming responses
onMounted(async () => {
  unlisten = await listen("llm-stream", (event) => {
    currentStreamingContent.value += event.payload as string;
  });
  
  // Load conversation list
  await loadConversations();

  // Listen for window resize
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
  window.removeEventListener('resize', handleResize);
});

function handleResize() {
  if (window.innerWidth <= 768) {
    sidebarOpen.value = false;
  }
}

// Toggle sidebar
function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value;
}

// Create new chat
function createNewChat() {
  currentConversation.value = null;
  if (window.innerWidth <= 768) {
    sidebarOpen.value = false;
  }
}

// Handle sending messages
async function handleSendMessage(message: string) {
  if (!message.trim()) return;
  
  isLoading.value = true;
  currentStreamingContent.value = "";
  
  try {
    if (currentConversation.value) {
      await continueConversation(currentConversation.value.id, message);
    } else {
      await startNewConversation(message);
      await continueConversation(currentConversation.value!.id, message);
    }
  } catch (error) {
    console.error("Error sending message:", error);
  } finally {
    isLoading.value = false;
  }
}

// Make key functions and state available to child components
provide('darkMode', darkMode);
provide('isLoading', isLoading);
provide('currentStreamingContent', currentStreamingContent);
provide('handleSendMessage', handleSendMessage);
</script>

<template>
  <div class="app-container" :class="{ 'dark-mode': darkMode }">
    <!-- Sidebar component -->
    <Sidebar 
      :conversations="conversations" 
      :currentConversationId="currentConversation?.id"
      :isOpen="sidebarOpen"
      @toggle="toggleSidebar"
      @load-conversation="loadConversation"
      @create-new="createNewChat"
      @delete-conversation="deleteConversation"
    />
    
    <!-- Main chat container -->
    <ChatContainer
      :conversation="currentConversation"
      :isLoading="isLoading"
      :streamingContent="currentStreamingContent"
      @toggle-sidebar="toggleSidebar"
      @send-message="handleSendMessage"
    />
  </div>
</template>

<style>
.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>