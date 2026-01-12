<script setup lang="ts">
  import {
    kPage,
    kNavbar,
    kLink,
  } from 'konsta/vue';

  import MessageList from './MessageList.vue';
  import MessageInput from './MessageInput.vue';
  import type { Conversation } from '../types';

  // Props
  defineProps<{
    conversation: Conversation | null;
    isLoading: boolean;
    streamingContent: string;
    isDark: boolean;
  }>();

  // Emits
  const emit = defineEmits<{
    (e: 'toggle-sidebar'): void;
    (e: 'send-message', message: string): void;
    (e: 'toggle-theme'): boolean;
  }>();

</script>

<template>
  <k-page class="min-h-screen flex flex-col">
    <k-navbar :title="conversation?.title || 'New Conversation'" class="ios:pb-4">
      <template #left>
        <k-link @click="emit('toggle-sidebar')">
          <i class="pi pi-bars"></i>
        </k-link>
      </template>
      <template #right>
        <k-link @click="emit('toggle-theme')">
          <i v-if="!isDark" class="pi pi-moon"></i>
          <i v-else class="pi pi-sun"></i>
        </k-link>
      </template>
    </k-navbar>
    
    <div class="flex-1 flex flex-col min-h-0">
      <MessageList
        :conversation="conversation"
        :streamingContent="streamingContent" 
        :isLoading="isLoading"
        class="flex-1"
      />
      <MessageInput
        :isLoading="isLoading"
        @send="emit('send-message', $event)"
      />
    </div>
  </k-page>
</template>
