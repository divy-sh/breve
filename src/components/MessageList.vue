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
  <div
    class="flex-1 overflow-y-auto flex flex-col min-h-0"
  >
    <k-block v-if="!props.conversation" class="flex-shrink-0">
      <h2 class="text-xl font-semibold">Hello There!</h2>
      <p class="text-sm">Start a new conversation by typing a message below.</p>
    </k-block>
    
    <template v-else>
      <k-block strong inset
        v-for="(message, index) in messages" :key="index" :class="['max-w-[80%] flex-shrink-0 m-4',
          message.role === 'user' ? 'self-end' : 'self-start'
        ]"
      >
        <div class="font-semibold text-sm">
          <span>{{ message.role === 'user' ? 'You' : 'Breve' }}</span>
        </div>
        <div class="whitespace-pre-wrap break-words">{{ message.content }}</div>
      </k-block>
    </template>
  </div>
</template>
