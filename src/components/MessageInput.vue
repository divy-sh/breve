<script setup lang="ts">
import { ref } from 'vue';

// Props
const props = defineProps<{
  isLoading: boolean;
}>();

// Emits
const emit = defineEmits<{
  (e: 'send', message: string): void;
}>();

// State
const inputText = ref("");

// Methods
function handleSubmit() {
  if (inputText.value.trim() && !props.isLoading) {
    emit('send', inputText.value);
    inputText.value = "";
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    handleSubmit();
  }
}
</script>

<template>
  <div class="input-area">
    <form @submit.prevent="handleSubmit">
      <textarea 
        v-model="inputText" 
        placeholder="Type your message..." 
        @keydown="handleKeydown"
        :disabled="isLoading"
        ref="textareaRef"
      ></textarea>
      <button type="submit" :disabled="isLoading || !inputText.trim()">
        {{ isLoading ? '...' : 'Send' }}
      </button>
    </form>
    <div class="input-hint">Press Enter to send, Shift+Enter for new line</div>
  </div>
</template>

<style scoped>
.input-area {
  padding: 1rem;
  border-top: 1px solid var(--primary-color);
}

.input-area form {
  display: flex;
  gap: 0.5rem;
}

.input-area textarea {
  flex: 1;
  padding: 0.75rem;
  border: 1px solid var(--primary-color);
  border-radius: 0.5rem;
  resize: none;
  height: 60px;
  font-family: inherit;
  font-size: 1rem;
  background-color: var(--background-color);
  color: var(--text-color);
}

.input-area button {
  padding: 0 1rem;
  background-color: var(--primary-color);
  color: var(--text-color);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.2s;
}

.input-area button:hover:not(:disabled) {
  background-color: var(--secondary-color);
}

.input-area button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.input-hint {
  margin-top: 0.5rem;
  font-size: 0.75rem;
  text-align: right;
  color: var(--secondary-color);
}
</style>