<script setup lang="ts">
  import { computed, ref, onMounted, watch } from 'vue';
  import {
    kBlock,
  } from 'konsta/vue';
  import type { Conversation } from '../types';

  const props = defineProps<{
    conversation: Conversation | null;
    streamingContent: string;
    isLoading: boolean;
  }>();

  const messagesContainer = ref<HTMLDivElement | null>(null);

  const messages = computed(() => {
    if (!props.conversation || !props.conversation.body) return [];
    const allMessages = [...props.conversation.body];
    if (props.streamingContent && props.isLoading) {
      allMessages.push({
        role: 'assistant',
        content: props.streamingContent
      });
    }
    
    return allMessages;
  });

  function scrollToBottom() {
    if (messagesContainer?.value) {
      setTimeout(() => {
        if (messagesContainer.value) {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
      }, 10);
    }
  }

  watch(() => props.conversation?.body?.length, scrollToBottom);
  // watch(() => props.streamingContent, scrollToBottom);

  onMounted(scrollToBottom);
</script>

<template>
  <div class="messages-container" ref="messagesContainer">
    <k-block v-if="!props.conversation" class="welcome-screen">
      <h2>Hello There!</h2>
      <p>Start a new conversation by typing a message below.</p>
    </k-block>
    
    <template v-else>
      <k-block inset strong
        v-for="(message, index) in messages" 
        :key="index"
        class="message"
        :class="message.role"
      >
        <div class="message-header">
          <span class="message-role">{{ message.role === 'user' ? 'You' : 'Breve' }}</span>
        </div>
        <div class="message-content">{{ message.content }}</div>
      </k-block>
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

.message {
  margin-bottom: 1rem;
  padding: 1rem;
  border-radius: 0.5rem;
  max-width: 80%;
}

.message.user {
  align-self: flex-end;
}

.message.assistant {
  align-self: flex-start;
}

.message-header {
  margin-bottom: 0.5rem;
  font-weight: 600;
  font-size: 0.875rem;
}
</style>
