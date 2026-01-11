<script setup lang="ts">
  import {
    kPage,
    kNavbar,
    kButton,
  } from 'konsta/vue';

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
  <k-page>
    <k-navbar :title="conversation?.title || 'New Conversation'">
      <template #left>
        <k-button clear @click="emit('toggle-sidebar')">
          <i class="pi pi-bars"></i>
        </k-button>
      </template>
    </k-navbar>
    
    <div class="flex-box">
      <MessageList 
        :conversation="conversation"
        :streamingContent="streamingContent" 
        :isLoading="isLoading"
      />
      <MessageInput class="fixed bottom-0"
        :isLoading="isLoading"
        @send="emit('send-message', $event)"
      />
    </div>
  </k-page>
</template>
