<script setup lang="ts">
import type { ConversationSummary } from '../types';

// Props
defineProps<{
  conversations: ConversationSummary[];
  currentConversationId?: string;
  isOpen: boolean;
}>();

// Emits
const emit = defineEmits<{
  (e: 'toggle'): void;
  (e: 'load-conversation', id: string): void;
  (e: 'create-new'): void;
}>();
</script>

<template>
  <aside class="sidebar" :class="{ 'open': isOpen }">
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
        :class="{ 'active': currentConversationId === convo.id }"
      >
        <span class="convo-title">{{ convo.title }}</span>
      </div>
      <div v-if="conversations.length === 0" class="no-conversations">
        No conversations yet
      </div>
    </div>
  </aside>
</template>

<style scoped>
/* Sidebar styles */
.sidebar {
  width: 280px;
  background-color: var(--sidebar-bg);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  transition: transform 0.3s ease;
  z-index: 10;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  color: white;
}

.new-chat-btn {
  margin: 1rem;
  padding: 0.75rem;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.2s;
}

.new-chat-btn:hover {
  background-color: var(--primary-hover);
}

.conversation-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.conversation-item {
  padding: 0.75rem;
  border-radius: 0.5rem;
  margin-bottom: 0.5rem;
  cursor: pointer;
  transition: background-color 0.2s;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: white;
}

.conversation-item:hover {
  background-color: var(--border-color);
}

.conversation-item.active {
  background-color: var(--primary-color);
  color: white;
}

.no-conversations {
  padding: 1rem;
  text-align: center;
  color: #64748b;
}

.sidebar-toggle {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-color);
}

/* Mobile styles */
@media (max-width: 768px) {
  .sidebar {
    position: absolute;
    height: 100%;
    transform: translateX(-100%);
  }
  
  .sidebar.open {
    transform: translateX(0);
  }
}
</style>