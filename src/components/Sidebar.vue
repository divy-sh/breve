<script setup lang="ts">

  import {
    kNavbar,
    kPanel,
    kButton,
    kLink,
    kMenuList,
    kMenuListItem,
    kPopover,
    kPage,
    kBlock
  } from 'konsta/vue';
  import type { ConversationSummary } from '../types';
  import { ref, watch } from 'vue';

  const props = defineProps<{
    conversations: ConversationSummary[];
    currentConversationId?: string;
    isOpen: boolean;
  }>();

  const emit = defineEmits<{
    (e: 'toggle'): void;
    (e: 'load-conversation', id: string): void;
    (e: 'create-new'): void;
    (e: 'delete-conversation', id: string): void;
    (e: 'set-config', key: string, value: any): void;
  }>();

  const openDropdownFor = ref<string | null>(null);
  const handleMenuClick = (id: string) => {
    openDropdownFor.value = openDropdownFor.value === id ? null : id;
  };

  const popoverOpened = ref(false);
  const popoverTargetRef = ref("");

  const openPopover = (targetRef: any) => {
    popoverTargetRef.value = targetRef;
    popoverOpened.value = true;
  };

  const deleteConversation = (id: string) => {
    emit('delete-conversation', id);
    openDropdownFor.value = null;
  };

  watch(() => props.isOpen, (val) => {
    if (!val) openDropdownFor.value = null;
  });

  watch(() => props.currentConversationId, () => {
    openDropdownFor.value = null;
  });
</script>

<template>
  <k-panel
    side="left"
    floating
    :opened="isOpen"
    @backdropclick="() => (emit('toggle'))" >
    <k-page class="overscroll-none">
      <k-navbar title="Chats">
        <template #right>
          <k-link @click="emit('toggle')">
            <i class="pi pi-times"></i>
          </k-link>
        </template>
      </k-navbar>
      <k-block>
        <k-button @click="emit('create-new'); emit('toggle')" class="new-chat-btn">
          <i class="pi pi-sparkles">  New Chat</i>
        </k-button>
      </k-block>
      <k-menu-list>
        <k-menu-list-item
          v-for="convo in conversations"
          :key="convo.id"
          :title="convo.title || 'Untitled Chat'"
          :active="isOpen && currentConversationId === convo.id"
          @click="emit('load-conversation', convo.id); emit('toggle'); emit('set-config', 'lastConversationId', convo.id)"
        >
          <template #after>
            <k-button clear @click.stop="handleMenuClick(convo.id), openPopover($event.currentTarget)">
              <i class="pi pi-ellipsis-h"></i>
            </k-button>
          </template>
        </k-menu-list-item>
      </k-menu-list>
      <div v-if="conversations.length === 0" class="no-conversations text-center">
        No conversations yet
      </div>
    </k-page>
  </k-panel>
  <k-popover
    :opened="popoverOpened"
    :target="popoverTargetRef"
    @backdropclick="() => (popoverOpened = false)"
    class="w-32 p-2" >
      <k-button clear
        @click="() => { popoverOpened = false; deleteConversation(openDropdownFor!); }" >
        Delete
      </k-button>
  </k-popover>
</template>
