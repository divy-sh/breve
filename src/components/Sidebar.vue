<script setup lang="ts">
import type { ConversationSummary } from '../types';
import { ref } from 'vue';
import SettingsPanel from './SettingsPanel.vue';

const showSettings = ref(false);
const toggleSettings = () => {
  showSettings.value = !showSettings.value;
};

// Props
const props = defineProps<{
  conversations: ConversationSummary[];
  currentConversationId?: string;
  isOpen: boolean;
}>();

// Local State
const openDropdownFor = ref<string | null>(null);

// Emits
const emit = defineEmits<{
  (e: 'toggle'): void;
  (e: 'load-conversation', id: string): void;
  (e: 'create-new'): void;
  (e: 'delete-conversation', id: string): void;
}>();

// Methods
const handleMenuClick = (id: string) => {
  console.log(id, id == props.currentConversationId)
  if (id !== props.currentConversationId) {
    emit('load-conversation', id);
  } else {
    openDropdownFor.value = openDropdownFor.value === id ? null : id;
  }
};
</script>

<template>
  <aside class="sidebar" :class="{ open: isOpen }">
    <div class="sidebar-header">
      <h2>Conversations</h2>
      <button @click="emit('toggle')" class="sidebar-toggle mobile-only">×</button>
    </div>

    <button @click="emit('create-new')" class="new-chat-btn">
      + New Chat
    </button>

    <div class="conversation-list">
      <div 
        v-for="convo in conversations" 
        :key="convo.id" 
        @click="emit('load-conversation', convo.id)"
        class="conversation-item"
        :class="{ active: currentConversationId === convo.id }"
      >
        <span class="convo-title">{{ convo.title }}</span>
        <div @click.stop="handleMenuClick(convo.id)">⋮</div>
        <div v-if="openDropdownFor === convo.id" class="dropdown">
          <div @click="emit('delete-conversation', convo.id)">Delete</div>
        </div>
      </div>

      <div v-if="conversations.length === 0" class="no-conversations">
        No conversations yet
      </div>
    </div>
    <div class="bottom-bar" @click="toggleSettings">
      ⚙️ Settings
    </div>
  <SettingsPanel v-if="showSettings" @close="showSettings = false" />
  </aside>
</template>

<style scoped>
/* Sidebar styles */
.sidebar {
  background-color: var(--background-color);
  width: 280px;
  border-right: 1px solid var(--primary-color);
  display: flex;
  flex-direction: column;
  z-index: 10;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--primary-color);
  color: var(--secondary-color);
}

.new-chat-btn {
  margin: 1rem;
  padding: 0.75rem;
  background-color: var(--primary-color);
  color: var(--text-color);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.2s;
}

.new-chat-btn:hover {
  background-color: var(--secondary-color);
}

.conversation-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.conversation-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  border-radius: 0.5rem;
  margin-bottom: 0.5rem;
  cursor: pointer;
  transition: background-color 0.2s;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-color);
  position: relative;
}

.conversation-item:hover {
  background-color: var(--primary-color);
}

.conversation-item.active {
  background-color: var(--primary-color);
  color: var(--text-color);
}

.no-conversations {
  padding: 1rem;
  text-align: center;
  color: var(--text-color);
}

.sidebar-toggle {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-color);
}

.dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  background-color: var(--primary-color);
  color: var(--text-color);
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 5;
}

.bottom-bar {
  border-top: 1px solid var(--primary-color);
  padding: 1rem;
  text-align: center;
  align-items: center;
  color: var(--text-color);
}

/* Mobile styles */
@media (max-width: 768px) {
  .sidebar {
    position: absolute;
    height: 100%;
    transform: translateX(-100%);
    transition: transform 0.3s ease;
  }

  .sidebar.open {
    transform: translateX(0);
  }
}
</style>
