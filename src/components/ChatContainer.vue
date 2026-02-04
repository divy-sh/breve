<script setup lang="ts">
  import {
    kPage,
    kNavbar,
    kLink,
    kPopover,
    kListItem,
  } from 'konsta/vue';

  import MessageList from './MessageList.vue';
  import MessageInput from './MessageInput.vue';
  import type { Conversation } from '../types';
  import { ref } from 'vue';
  import Settings from './settings.vue';

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


  const popoverOpened = ref(false);
  const popoverTargetRef = ref("");

  const openPopover = (targetRef: any) => {
    popoverTargetRef.value = targetRef;
    popoverOpened.value = true;
  };

  const openSettings = ref(false);
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
        <k-link @click="openPopover($event.currentTarget)">
          <i class="pi pi-cog"></i>
        </k-link>
      </template>
    </k-navbar>
    
    <k-popover
      :opened="popoverOpened"
      :target="popoverTargetRef"
      @backdropclick="() => (popoverOpened = false)">

      <k-list-item link title="Toggle Theme" @click="emit('toggle-theme'); popoverOpened = false;">
        <template #media>
          <i v-if="!isDark" class="pi pi-moon mr-2"></i>
          <i v-else class="pi pi-sun mr-2"></i>
        </template>
      </k-list-item>

      <k-list-item title="Settings" link @click="() => { openSettings = true; popoverOpened = false; }">
        <template #media>
          <i class="pi pi-cog mr-2"></i>
        </template>
      </k-list-item>
      
      <k-list-item title="Close" link @click="() => (popoverOpened = false)">
        <template #media>
          <i class="pi pi-times mr-2"></i>
        </template>
      </k-list-item>
    </k-popover>

    <Settings
      :openSettings="openSettings"
      @close="openSettings = false"
     />

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
