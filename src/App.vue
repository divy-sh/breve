<script setup lang="ts">
import { kApp } from 'konsta/vue';
import { ref, onMounted, onUnmounted, provide } from "vue";
import { listen } from "@tauri-apps/api/event";

import Sidebar from "./components/Sidebar.vue";
import ChatContainer from "./components/ChatContainer.vue";
import { useConversations } from "./composables/useConversations";
import { useSettings } from './composables/useSettings';
import ModelsDownload from './components/ModelsDownload.vue';

const {
  conversations, 
  currentConversation,
  modelStatus,
  loadConversations, 
  loadConversation, 
  startNewConversation,
  continueConversation,
  deleteConversation,
  getModelStatus
} = useConversations();

const {
  getConfig,
  setConfig
} = useSettings();

const theme = ref<'material' | 'ios'>('material');
const isDark = ref(false);

// TODO: refactor to also use conversation id so that streaming content 
// is properly associated with the conversation when switching between them while streaming
const currentStreamingContent = ref("");
const isLoading = ref(false);
const sidebarOpen = ref(false);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  modelStatus.value = await getModelStatus();
  getConfig('darkMode').then((value) => {
    setDark(value === 'true');
  });

  unlisten = await listen("llm-stream", (event) => {
    currentStreamingContent.value += event.payload as string;
  });

  loadConversations().catch(console.error);

  loadConversation(await getConfig('lastConversationId')).catch(console.error);

});

onUnmounted(() => {
  unlisten?.();
});

function setDark(value: boolean) {
  const html = document.documentElement;
  if (value) {
    html.classList.add('dark');
  } else {
    html.classList.remove('dark');
  }
  isDark.value = value;
  setConfig('darkMode', value.toString()).catch(console.error);
}

function createNewChat() {
  currentConversation.value = null;
}

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value;
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

provide('isLoading', isLoading);
provide('currentStreamingContent', currentStreamingContent);
provide('handleSendMessage', handleSendMessage);
provide('AppTheme', {
  theme,
  isDark,
  setDark,
});
</script>

<template>
  <k-app :theme="theme">
    <ModelsDownload v-if="modelStatus === 'UNSET'" />

    <Sidebar v-if="modelStatus === 'SET'"
      :conversations="conversations"
      :currentConversationId="currentConversation?.id"
      :isOpen="sidebarOpen"
      @toggle="toggleSidebar"
      @load-conversation="loadConversation"
      @create-new="createNewChat"
      @delete-conversation="deleteConversation"
      @set-config="setConfig"
    />

    <ChatContainer v-if="modelStatus === 'SET'"
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
