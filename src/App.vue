<script setup lang="ts">
import { kApp } from 'konsta/vue';
import { ref, onMounted, onUnmounted, provide } from "vue";
import { listen } from "@tauri-apps/api/event";

import Sidebar from "./components/Sidebar.vue";
import ChatContainer from "./components/ChatContainer.vue";
import { useConversations } from "./composables/useConversations";
import { useSettings } from './composables/useSettings';
import ModelsDownload from './components/ModelsDownload.vue';
import type { StreamPayload } from './types';

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

const streamingContent = ref<StreamPayload>({ id: '', content: '' });
const isLoading = ref(false);
const sidebarOpen = ref(false);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  modelStatus.value = await getModelStatus();
  getConfig('darkMode').then((value) => {
    setDark(value === 'true');
  });

  unlisten = await listen<{id: string, content: string}>("llm-stream", (event) => {
    streamingContent.value = {
      id: event.payload.id,
      content: streamingContent.value.content + event.payload.content
    };
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
  streamingContent.value = {
    id: '',
    content: '',
  }

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
provide('currentStreamingContent', streamingContent);
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
      :streamingContent="streamingContent"
      :is-dark="isDark"
      @toggle-theme="setDark(!isDark)"
      @toggle-sidebar="toggleSidebar"
      @send-message="handleSendMessage"
    />
  </k-app>
</template>
