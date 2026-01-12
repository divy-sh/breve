<script setup lang="ts">
  import { computed, ref, onMounted, watch, nextTick } from 'vue';
  import {
    kMessages,
    kMessage,
    kMessagesTitle,
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
      nextTick(() => {
        if (messagesContainer.value) {
          messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
        }
      });
    }
  }

  watch(() => props.conversation?.body?.length, scrollToBottom);
  watch(() => props.streamingContent, scrollToBottom);

  onMounted(scrollToBottom);

  const dateFormatter = new Intl.DateTimeFormat('en-US', {
    weekday: 'long',
    month: 'short',
    day: 'numeric',
  });

  const currentDate = new Date();
  const currentDay = dateFormatter.format(currentDate);
</script>

<template>
  <div class="flex-1 overflow-y-auto flex flex-col min-h-0 p-4" ref="messagesContainer">
    <k-messages v-if="props.conversation">
      <k-messages-title>
        <b>{{ currentDay }}</b>
      </k-messages-title>
      <k-message 
        v-for="(message, index) in messages"
        :key="index"
        :type="message.role === 'user' ? 'sent' : 'received'"
        :name="message.role === 'user' ? 'You' : 'Breve'"
        :colors="{
          bubbleReceivedMd: 'bg-[#e5e5ea] dark:bg-[#252525]'
        }"
        class="max-w-[90%]"
      >
        <template #text>
          <div class="whitespace-pre-wrap break-word text-sm" v-html="message.content"></div>
        </template>
      </k-message>
    </k-messages>

    <div v-else class="flex items-center justify-center h-full">
      <div class="text-center">
        <h2 class="text-xl font-semibold mb-2">Hello There!</h2>
        <p class="text-sm">Start a new conversation by typing a message below.</p>
      </div>
    </div>
  </div>
</template>
