<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from "vue";
import { listen } from "@tauri-apps/api/event";

import Sidebar from "./components/Sidebar.vue";
import ChatContainer from "./components/ChatContainer.vue";
import { useConversations } from "./composables/useConversations";
import DownloadingModel from "./components/DownloadingModel.vue";
import DownloadPage from "./components/DownloadPage.vue";
import SettingsPanel from './components/SettingsPanel.vue';

const showSettings = ref(false);
const toggleSettings = () => {
  showSettings.value = !showSettings.value;
};
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
const downloadStatus = ref("");

// Unlisten function for the event listener
let unlisten: (() => void) | null = null;

let unlisten2: (() => void) | null = null;

// Setup event listener for LLM streaming responses
onMounted(async () => {
  unlisten = await listen("llm-stream", (event) => {
    currentStreamingContent.value += event.payload as string;
  });
  
  unlisten2 = await listen("download-status", (event) => {
    downloadStatus.value = event.payload as string;
  });

  // Load conversation list in background so the UI can render immediately.
  loadConversations().catch((e) => console.error('loadConversations failed', e));
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
  if (unlisten2) {
    unlisten2();
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
    <DownloadPage v-if="downloadStatus === 'awaitingUser'"/>
    <DownloadingModel v-if="downloadStatus === 'downloading'"/>
    <!-- Sidebar component -->
    <Sidebar v-if="downloadStatus === 'downloaded'"
      :conversations="conversations" 
      :currentConversationId="currentConversation?.id"
      :isOpen="sidebarOpen"
      :showSettings="showSettings"
      @toggle-settings="toggleSettings"
      @toggle="toggleSidebar"
      @load-conversation="loadConversation"
      @create-new="createNewChat"
      @delete-conversation="deleteConversation"
    />
    <!-- Main chat container -->
    <ChatContainer v-if="downloadStatus === 'downloaded'"
      :conversation="currentConversation"
      :isLoading="isLoading"
      :streamingContent="currentStreamingContent"
      @toggle-sidebar="toggleSidebar"
      @send-message="handleSendMessage"
    />
    <SettingsPanel v-if="showSettings" @close="showSettings = false" />
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