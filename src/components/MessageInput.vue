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
}

function handleSend() {
  if (messageText.value.trim() && !props.isLoading) {
    emit('send', messageText.value.trim())
    messageText.value = ''
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
      class="bg-transparent"
    >
      <template #right>
        <k-link
          @click="handleSend"
          :class="[
            'px-3 py-2 font-semibold',
            props.isLoading || !messageText.trim()
              ? 'opacity-50 pointer-events-none'
              : 'text-primary'
          ]"
        >
          <!-- You can use your own icon here -->
          <i class="pi pi-send"></i>
        </k-link>
      </template>
    </k-messagebar>
  </k-block>
</template>
