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
  continueConversation
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
  
  // Listen for dark mode changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    darkMode.value = e.matches;
  });
  
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
      await continueConversation(currentConversation.value.id, message);
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
:root {
  --primary-color: #3b82f6;
  --primary-hover: #2563eb;
  --bg-color: #f8fafc;
  --text-color: #1e293b;
  --border-color: #e2e8f0;
  --sidebar-bg: #f1f5f9;
  --message-user-bg: #e0f2fe;
  --message-assistant-bg: #f8fafc;
  --input-bg: #ffffff;
  --shadow-color: rgba(0, 0, 0, 0.05);
  
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.5;
  color: var(--text-color);
  background-color: var(--bg-color);
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

/* Dark mode styles */
.dark-mode {
  --primary-color: #60a5fa;
  --primary-hover: #93c5fd;
  --bg-color: #0f172a;
  --text-color: #f1f5f9;
  --border-color: #334155;
  --sidebar-bg: #1e293b;
  --message-user-bg: #1e40af;
  --message-assistant-bg: #1e293b;
  --input-bg: #1e293b;
  --shadow-color: rgba(0, 0, 0, 0.3);
}

@media (max-width: 768px) {
  .desktop-hidden {
    display: initial;
  }
  
  .mobile-only {
    display: initial;
  }
}

@media (min-width: 769px) {
  .desktop-hidden {
    display: none;
  }
  
  .mobile-only {
    display: none;
  }
}
</style>