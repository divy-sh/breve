<script setup lang="ts">

  import {
    kNavbar,
    kPanel,
    kBlock,
    kButton,
    kMenuList,
    kMenuListItem,
    kList,
    kListItem,
    kPopover
  } from 'konsta/vue';
  import type { ConversationSummary } from '../types';
  import { ref, watch } from 'vue';

  // Props
  const props = defineProps<{
    conversations: ConversationSummary[];
    currentConversationId?: string;
    isOpen: boolean;
  }>();

  // Emits
  const emit = defineEmits<{
    (e: 'toggle'): void;
    (e: 'load-conversation', id: string): void;
    (e: 'create-new'): void;
    (e: 'delete-conversation', id: string): void;
  }>();

  const openDropdownFor = ref<string | null>(null);
  // Methods
  const handleMenuClick = (id: string) => {
    // Only toggle the dropdown; don't change conversation selection.
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

  // Clear any open dropdown when sidebar closes
  watch(() => props.isOpen, (val) => {
    if (!val) openDropdownFor.value = null;
  });

  // Clear dropdown when current conversation changes
  watch(() => props.currentConversationId, () => {
    openDropdownFor.value = null;
  });
</script>

<template>
  <k-panel
    side="left"
    floating
    :opened="isOpen"
    @backdropclick="() => (emit('toggle'))"
  >
    <k-navbar title="Chats">
      <template #right>
        <k-button clear @click="emit('toggle')">
          <i class="pi pi-times"></i>
        </k-button>
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
            @click="emit('load-conversation', convo.id); emit('toggle')"
        >
          <template #after>
            <k-button clear @click.stop="handleMenuClick(convo.id), openPopover($event.currentTarget)">
              <i class="pi pi-ellipsis-v"></i>
            </k-button>
          </template>
        </k-menu-list-item>
      </k-menu-list>
      <div v-if="conversations.length === 0" class="no-conversations text-center">
        No conversations yet
      </div>
  </k-panel>
  <k-popover
      :opened="popoverOpened"
      :target="popoverTargetRef"
      @backdropclick="() => (popoverOpened = false)"
    >
      <k-list nested>
        <k-list-item
          title="Delete"
          @click="() => { popoverOpened = false; deleteConversation(openDropdownFor!); }"
        />
      </k-list>
    </k-popover>
</template>
