<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { Window } from "@tauri-apps/api/window";

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
const textareaRef = ref<HTMLTextAreaElement | null>(null);

// Methods
function handleSubmit() {
  if (inputText.value.trim() && !props.isLoading) {
    emit('send', inputText.value);
    inputText.value = "";
    nextTick(autoResize); // reset height after clearing
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    handleSubmit();
  }
}

function autoResize() {
  const el = textareaRef.value;
  if (!el) return;
  el.style.height = 'auto';
  el.style.height = el.scrollHeight + 'px';
}

onMounted(async () => {
  const appWindow = Window.getCurrent();
  autoResize();
  nextTick(() => {
    setTimeout(() => {
      textareaRef.value?.focus();
    }, 50)
  });
    appWindow.onFocusChanged(({ payload } : any) => {
    if (payload) {
      textareaRef.value?.focus();
    }
  });
});
watch(inputText, autoResize);
</script>

<template>
  <div class="input-area">
    <form @submit.prevent="handleSubmit">
      <textarea 
        v-model="inputText" 
        placeholder="Type your message..." 
        @keydown="handleKeydown"
        @input="autoResize"
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
  overflow: hidden;
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