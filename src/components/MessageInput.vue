<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Window } from '@tauri-apps/api/window'
import { kBlock, kMessagebar, kLink } from 'konsta/vue'

// Props
const props = defineProps<{
  isLoading: boolean
}>()

// Emits
const emit = defineEmits<{
  (e: 'send', message: string): void
}>()

// State
const messageText = ref('')

// Methods
function onMessageTextChange(e: any) {
  messageText.value = e.target.value
  const el = e.target

  el.style.height = 'auto'
  el.style.height = el.scrollHeight + 'px'
}

function handleSend() {
  if (messageText.value.trim() && !props.isLoading) {
    emit('send', messageText.value.trim())
    messageText.value = ''
  }

  const textarea = document.querySelector('textarea')
  if (textarea) {
    textarea.style.height = 'auto'
  }
}

// Focus handling with Tauri
onMounted(() => {
  const appWindow = Window.getCurrent()

  appWindow.onFocusChanged(({ payload }: any) => {
    if (payload) {
      document.querySelector('textarea')?.focus()
    }
  })
})
</script>

<template>
  <k-block class="m-0 p-0">
    <k-messagebar
      placeholder="Type your message..."
      :disabled="props.isLoading"
      :value="messageText"
      @input="onMessageTextChange"
      class="
        [&_.h-14]:h-auto
        [&_.h-12]:h-auto

        [&_.flex]:py-2
        [&_.flex]:items-end

        [&_textarea]:min-h-48px]
        [&_textarea]:max-h-[120px]
        [&_textarea]:overflow-y-auto"
      >
        <template #right>
          <k-link
            @click="handleSend"
            :class="[
              'material:px-4 material:py-7 ios:p-4 font-semibold',
              props.isLoading || !messageText.trim()
                ? 'opacity-50 pointer-events-none'
                : 'text-primary'
            ]">
            <i class="pi pi-send"></i>
          </k-link>
        </template>
      </k-messagebar>
  </k-block>
</template>
