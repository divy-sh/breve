<script setup lang="ts">
  import { computed, ref, onMounted, watch, nextTick } from 'vue';
  import { kMessages, kMessage, kMessagesTitle } from 'konsta/vue';
  import type { Conversation, StreamPayload } from '../types';
  import markdownit from 'markdown-it';

  const md = markdownit({ html: true, linkify: true, typographer: true });

  const props = defineProps<{
    conversation: Conversation | null;
    streamingContent: StreamPayload;
    isLoading: boolean;
  }>();

  const messagesContainer = ref<HTMLDivElement | null>(null);

  const messages = computed(() => {
    if (!props.conversation?.body) return [];
    const allMessages = [...props.conversation.body];
    if (props.streamingContent && props.isLoading && props.streamingContent.id == props.conversation.id) {
      allMessages.push({ role: 'assistant', content: props.streamingContent.content });
    }
    return allMessages;
  });

  const scrollToBottom = () => {
    nextTick(() => {
      if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
      }
    });
  };

  watch(() => props.conversation?.body?.length, scrollToBottom);
  watch(() => props.streamingContent, scrollToBottom);
  onMounted(scrollToBottom);

  const currentDay = new Intl.DateTimeFormat('en-US', {
    weekday: 'long', month: 'short', day: 'numeric',
  }).format(new Date());
</script>

<template>
  <div class="flex-1 overflow-y-auto flex flex-col p-4" ref="messagesContainer">
    <k-messages v-if="props.conversation">
      <k-messages-title><b>{{ currentDay }}</b></k-messages-title>
      <k-message 
        v-for="(message, index) in messages"
        :key="index"
        :type="message.role === 'user' ? 'sent' : 'received'"
        :name="message.role === 'user' ? 'You' : 'Breve'"
        :colors="{ bubbleReceivedMd: 'bg-[#e5e5ea] dark:bg-[#252525]' }"
        class="max-w-[90%]"
      >
        <template #text>
          <div class="grid min-w-0">
            <div 
              class="text-sm break-words overflow-hidden [&_pre]:overflow-x-auto [&_pre]:whitespace-pre 
              [&_pre]:p-3 [&_pre]:bg-black/5 dark:[&_pre]:bg-white/10 [&_pre]:rounded-md"
              v-html="md.render(message.content)"
            ></div>
          </div>
        </template>
      </k-message>
    </k-messages>

    <div v-else class="flex items-center justify-center h-full text-center">
      <div>
        <h2 class="text-xl font-semibold mb-2">Hello There!</h2>
        <p class="text-sm">Start a new conversation by typing a message below.</p>
      </div>
    </div>
  </div>
</template>