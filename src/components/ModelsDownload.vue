<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { kBlock, List, ListItem, Button, Dialog } from 'konsta/vue';
import { listen } from '@tauri-apps/api/event';
import { useConversations } from '../composables/useConversations';

const {
  modelStatus,
  availableModels,
  downloadedModels,
  defaultModel,
  getAvailableModels,
  listDownloadedModels,
  downloadModel,
  deleteModel,
  setDefaultModel,
  getDefaultModel,
  getModelStatus,
} = useConversations();

const downloadingModel = ref<string | null>(null);
const downloadProgress = ref(0);
const showDeleteConfirm = ref(false);
const modelToDelete = ref<string | null>(null);

const isDefault = (name: string) => defaultModel.value === name;
const isDownloaded = (name: string) => downloadedModels.value.includes(name);
const isDownloading = (name: string) => downloadingModel.value === name;


let unlistenProgress: (() => void) | null = null;

async function refreshVariables() {
  availableModels.value = await getAvailableModels();
  downloadedModels.value = await listDownloadedModels();
  defaultModel.value = await getDefaultModel();
  modelStatus.value = await getModelStatus();
}

async function onDownload(name: string) {
  if (downloadingModel.value != null) return;

  downloadingModel.value = name;
  downloadProgress.value = 0;

  try {
    await downloadModel(name);
    await refreshVariables();
  } catch (err) {
    console.error("Download failed:", err);
    alert(`Failed to download model: ${err}`);
  } finally {
    downloadingModel.value = null;
    downloadProgress.value = 0;
  }
}

async function onSetDefault(name: string) {
  try {
    await setDefaultModel(name);
    await refreshVariables();    
  } catch (err) {
    console.error("Set default failed:", err);
    alert(`Failed to set default model: ${err}`);
  }
}

function confirmDelete(name: string) {
  modelToDelete.value = name;
  showDeleteConfirm.value = true;
}

async function proceedDelete() {
  if (!modelToDelete.value) return;
  await deleteModel(modelToDelete.value);
  await refreshVariables();
  showDeleteConfirm.value = false;
  modelToDelete.value = null;
}

onMounted(async () => {
  await refreshVariables();

  unlistenProgress = await listen('download-progress', (event) => {
    downloadProgress.value = event.payload as number;
  });
});

onUnmounted(() => {
  unlistenProgress?.();
});
</script>

<template>
  <k-block strong>
    <h3 class="text-lg font-semibold mb-4">Models</h3>

    <List v-if="Object.keys(availableModels).length > 0">
      <ListItem
        v-for="name in Object.keys(availableModels)"
        :key="name"
        :title="name"
      >
        <template v-if="isDownloaded(name)" #after>
          <div class="flex gap-2">
            <Button small outline @click="onSetDefault(name)" :disabled="isDefault(name)">
              {{ isDefault(name) ? 'Default' : 'Select' }}
            </Button>
            <Button small outline color="red" @click="confirmDelete(name)" :disabled="isDefault(name)">
              Delete
            </Button>
          </div>
        </template>

        <template v-else-if="isDownloading(name)" #after>
          <div class="flex flex-col gap-1 w-32">
            <div class="text-xs text-gray-600">{{ Math.round(downloadProgress) }}%</div>
            <div class="w-full bg-gray-200 rounded h-2">
              <div class="bg-blue-500 h-2 rounded transition-all" :style="{ width: downloadProgress + '%' }"></div>
            </div>
          </div>
        </template>

        <template v-else #after>
          <Button small outline @click="onDownload(name)" :disabled="downloadingModel !== null">Download</Button>
        </template>
      </ListItem>
    </List>

    <p v-else class="text-gray-600 text-center py-4">No available models.</p>
  </k-block>

  <Dialog :opened="showDeleteConfirm" title="Confirm Delete" @backdrop-click="showDeleteConfirm = false">
    <p class="mb-4">Are you sure you want to delete <strong>{{ modelToDelete }}</strong>?</p>
    <div class="flex gap-2">
      <Button @click="showDeleteConfirm = false" outline>Cancel</Button>
      <Button @click="proceedDelete" color="red">Delete</Button>
    </div>
  </Dialog>
</template>
