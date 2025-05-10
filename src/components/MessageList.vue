<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue';
import type { Conversation, Message } from '../types';

// Props
const props = defineProps<{
  conversation: Conversation | null;
  streamingContent: string;
  isLoading: boolean;
}>();

// Refs
const messagesContainer = ref<HTMLDivElement | null>(null);

// Computed properties
const messages = computed(() => {
  if (!props.conversation) return [];
  
  const allMessages = [...props.conversation.messages];
  
  // Add streaming message if there is content
  if (props.streamingContent && props.isLoading) {
    allMessages.push({
      role: 'assistant',
      content: props.streamingContent
    });
  }
  
  return allMessages;
});

// Auto-scroll to bottom when new messages are added
function scrollToBottom() {
  if (messagesContainer.value) {
    setTimeout(() => {
      if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
      }
    }, 10);
  }
}

// Watch for changes that should trigger a scroll
watch(() => props.conversation?.messages.length, scrollToBottom);
watch(() => props.streamingContent, scrollToBottom);

onMounted(scrollToBottom);
</script>

<template>
  <div class="messages-container" ref="messagesContainer">
    <div v-if="!conversation" class="welcome-screen">
      <h2>Welcome to your LLM Chat Application</h2>
      <p>Start a new conversation by typing a message below.</p>
    </div>
    
    <template v-else>
      <div 
        v-for="(message, index) in messages" 
        :key="index"
        class="message"
        :class="message.role"
      >
        <div class="message-header">
          <span class="message-role">{{ message.role === 'user' ? 'You' : 'Assistant' }}</span>
        </div>
        <div class="message-content">{{ message.content }}</div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  scroll-behavior: smooth;
}

.welcome-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  color: #64748b;
}

.welcome-screen h2 {
  margin-bottom: 1rem;
}

.message {
  margin-bottom: 1rem;
  padding: 1rem;
  border-radius: 0.5rem;
  max-width: 80%;
  box-shadow: 0 1px 3px var(--shadow-color);
}

.message.user {
  background-color: var(--message-user-bg);
  align-self: flex-end;
  color: white;
}

.message.assistant {
  background-color: var(--message-assistant-bg);
  align-self: flex-start;
  border: 1px solid var(--border-color);
}

.message-header {
  margin-bottom: 0.5rem;
  font-weight: 600;
  font-size: 0.875rem;
}

.message-content {
  white-space: pre-wrap;
}
</style>