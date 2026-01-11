<script setup lang="ts">
import { kApp } from 'konsta/vue';
import { ref, onMounted, onUnmounted, provide } from "vue";
import { listen } from "@tauri-apps/api/event";

import Sidebar from "./components/Sidebar.vue";
import ChatContainer from "./components/ChatContainer.vue";
import { useConversations } from "./composables/useConversations";
import DownloadingModel from "./components/DownloadingModel.vue";
import DownloadPage from "./components/DownloadPage.vue";

const {
  conversations, 
  currentConversation, 
  loadConversations, 
  loadConversation, 
  startNewConversation,
  continueConversation,
  deleteConversation
} = useConversations();

const theme = ref<'material' | 'ios'>('material');
const isDark = ref(false);

function setDark(value: boolean) {
  const html = document.documentElement;
  if (value) {
    html.classList.add('dark');
  } else {
    html.classList.remove('dark');
  }
  isDark.value = value;
}

onMounted(() => {
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  setDark(prefersDark);
});

provide('AppTheme', {
  theme,
  isDark,
  setDark,
});

const currentStreamingContent = ref("");
const isLoading = ref(false);
const downloadStatus = ref("");
const sidebarOpen = ref(false);

let unlisten: (() => void) | null = null;
let unlisten2: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen("llm-stream", (event) => {
    currentStreamingContent.value += event.payload as string;
  });

  unlisten2 = await listen("download-status", (event) => {
    downloadStatus.value = event.payload as string;
  });

  loadConversations().catch(console.error);
});

onUnmounted(() => {
  unlisten?.();
  unlisten2?.();
});

function createNewChat() {
  currentConversation.value = null;
}

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

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value;
}

provide('isLoading', isLoading);
provide('currentStreamingContent', currentStreamingContent);
provide('handleSendMessage', handleSendMessage);
</script>

<template>
  <k-app :theme="theme" safe-areas>
    <DownloadPage v-if="downloadStatus === 'awaitingUser'" />
    <DownloadingModel v-if="downloadStatus === 'downloading'" />
    <Sidebar v-if="downloadStatus === 'downloaded'"
      :conversations="conversations"
      :currentConversationId="currentConversation?.id"
      :isOpen="sidebarOpen"
      @toggle="toggleSidebar"
      @load-conversation="loadConversation"
      @create-new="createNewChat"
      @delete-conversation="deleteConversation"
    />

    <ChatContainer v-if="downloadStatus === 'downloaded'"
      :conversation="currentConversation"
      :isLoading="isLoading"
      :streamingContent="currentStreamingContent"
      :is-dark="isDark"
      @toggle-theme="setDark(!isDark)"
      @toggle-sidebar="toggleSidebar"
      @send-message="handleSendMessage"
    />
  </k-app>
</template>
