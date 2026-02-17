<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { kBlock, kBlockTitle, kButton, Dialog, kProgressbar, kCard } from 'konsta/vue';
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

// ADDED THESE TWO:
const expandedModels = ref<string[]>([]);
const toggleExpand = (name: string) => {
  if (expandedModels.value.includes(name)) {
    expandedModels.value = expandedModels.value.filter(m => m !== name);
  } else {
    expandedModels.value.push(name);
  }
};

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

  <k-block v-if="Object.keys(availableModels).length > 0">
    <k-block-title>Models</k-block-title>
    <k-card 
      v-for="name in Object.keys(availableModels)" 
      :key="name"
      footer-divider
    >
      <template #header>
        <h3>{{ name }}</h3>
      </template>

      <div v-if="isDownloading(name)" class="mt-4">
        <div class="flex justify-between text-xs mb-1">
          <span>Downloading...</span>
          <span>{{ Math.round(downloadProgress) }}%</span>
        </div>
        <k-progressbar :progress="downloadProgress / 100" />
      </div>
      
      <p>Params: {{ availableModels[name]['params' as any] }}</p>

      <div v-if="expandedModels.includes(name)">
        <p><strong>Status:</strong> {{ isDownloaded(name) ? 'Downloaded' : 'Available' }}</p>
      </div>

      <template #footer>
        <div class="flex w-full -m-2">
          <template v-if="isDownloaded(name)">
            <k-button clear class="w-1/3 rounded-none" @click="onSetDefault(name)" :disabled="isDefault(name)">
              <i :class="isDefault(name) ? 'pi pi-check-circle' : 'pi pi-play-circle'"></i>
            </k-button>
            <k-button clear class="w-1/3 rounded-none text-red-500" @click="confirmDelete(name)" :disabled="isDefault(name)">
              <i class="pi pi-trash"></i>
            </k-button>
          </template>

          <template v-else>
            <k-button clear class="w-2/3 ounded-none" @click="onDownload(name)" :disabled="downloadingModel !== null">
              <i class="pi pi-download mr-2"></i> Download
            </k-button>
          </template>

          <k-button clear class="w-1/3 rounded-none" @click="toggleExpand(name)">
            <i :class="expandedModels.includes(name) ? 'pi pi-angle-up' : 'pi pi-angle-down'"></i>
          </k-button>
        </div>
      </template>
    </k-card>
  </k-block>
  
  <Dialog :opened="showDeleteConfirm" title="Confirm Delete" @backdrop-click="showDeleteConfirm = false">
    <p class="mb-4">Are you sure you want to delete <strong>{{ modelToDelete }}</strong>?</p>
    <div class="flex gap-2">
      <k-button @click="showDeleteConfirm = false" outline>Cancel</k-button>
      <k-button @click="proceedDelete" class="k-color-brand-red">Delete</k-button>
    </div>
  </Dialog>
</template>