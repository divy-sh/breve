<script setup lang="ts">
  import { ref, onMounted, watch, nextTick } from 'vue';
  import { Window } from "@tauri-apps/api/window";
  import {
    kBlock
  } from 'konsta/vue';

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
  <k-block class="m-0 p-4 flex-shrink-0 bg-transparent" strong inset>
    <form @submit.prevent="handleSubmit" class="flex gap-2 items-end">
      <textarea
        v-model="inputText"
        placeholder="Type your message..."
        @keydown="handleKeydown"
        @input="autoResize"
        :disabled="isLoading"
        ref="textareaRef"
        class="flex-1 p-3 border rounded resize-none h-14 bg-transparent text-current overflow-hidden"
      ></textarea>
      <button
        type="submit"
        :disabled="isLoading || !inputText.trim()"
        class="px-4 py-2 bg-primary text-white rounded font-semibold disabled:opacity-50"
      >
        {{ isLoading ? '...' : 'Send' }}
      </button>
    </form>
    <div class="mt-2 text-xs text-right">Press Enter to send, Shift+Enter for new line</div>
  </k-block>
</template>

