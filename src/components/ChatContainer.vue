<script setup lang="ts">
import MessageList from './MessageList.vue';
import MessageInput from './MessageInput.vue';
import type { Conversation } from '../types';

// Props
defineProps<{
  conversation: Conversation | null;
  isLoading: boolean;
  streamingContent: string;
}>();

// Emits
const emit = defineEmits<{
  (e: 'toggle-sidebar'): void;
  (e: 'send-message', message: string): void;
}>();

</script>

<template>
  <main class="main-content">
    <!-- Top bar -->
    <div class="top-bar">
      <button @click="emit('toggle-sidebar')" class="sidebar-toggle desktop-hidden">
        ☰
      </button>
      <h2>{{ conversation?.title || 'New Conversation' }}</h2>
    </div>
    
    <!-- Messages container -->
    <MessageList 
      :conversation="conversation"
      :streamingContent="streamingContent" 
      :isLoading="isLoading"
    />
    
    <!-- Input area -->
    <MessageInput 
      :isLoading="isLoading"
      @send="emit('send-message', $event)"
    />
  </main>
</template>

<style scoped>
/* Main content styles */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  color: var(--text-color);
}

.top-bar {
  padding: 1rem;
  border-bottom: 1px solid var(--primary-color);
  display: flex;
  align-items: center;
  color: var(--secondary-color);
}

.sidebar-toggle {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--secondary-color);
  padding-right: 0.5rem;
}
</style>