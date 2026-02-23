<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { kChip, kBlockTitle, kButton, Dialog, kProgressbar, kList, kListItem } from 'konsta/vue';
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
const isSettingDefault = ref<string | null>(null);
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
    isSettingDefault.value = name;
    await setDefaultModel(name);
    await refreshVariables();    
  } catch (err) {
    console.error("Set default failed:", err);
  } finally {
    isSettingDefault.value = null;
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
  <k-block-title>Models</k-block-title>
  <k-list strong inset dividers>
    <k-list-item v-for="name in Object.keys(availableModels)">
      <!-- Title -->
      <template #title>
        {{ availableModels[name]['name' as any] }}
      </template>
      
      <!-- After title -->
      <template #after>
        {{ availableModels[name]['params' as any] }}
      </template>
      
      <!-- Below title -->
      <template #subtitle>
        
        <!-- Download status -->
        <div v-if="isDownloading(name)" >
          <div class="flex justify-between items-center mb-1">
            <span>Downloading</span>
            <span class="text-xs font-mono">{{ Math.round(downloadProgress) }}%</span>
          </div>
          <k-progressbar :progress="downloadProgress / 100" />
        </div>
        
        <!-- expanded view -->
        <div v-if="expandedModels.includes(name)">
          <k-chip class="m-0.5 max-w-full">
            <span class="truncate">Status: {{ isDownloaded(name) ? 'Downloaded' : 'Available' }}</span>
          </k-chip>

          <k-chip class="m-0.5 max-w-full">
            <span class="truncate">{{ availableModels[name]['repo' as any] }}</span>
          </k-chip>

          <k-chip class="m-0.5 max-w-full">
            <span class="truncate">Size: {{ availableModels[name]['size' as any] }}</span>
          </k-chip>
        </div>
        
        <!-- buttons to show when the model is downloaded -->
        <template v-if="isDownloaded(name)">
          <k-button clear inline @click="onSetDefault(name)" :disabled="isDefault(name)" class="w-1/3">
            <i v-if="isSettingDefault === name" class="pi pi-spinner pi-spin"></i>
            <i v-else :class="isDefault(name) ? 'pi pi-check-circle' : 'pi pi-play-circle'"></i>
          </k-button>
          <k-button clear inline @click="confirmDelete(name)" :disabled="isDefault(name)" class="w-1/3">
            <i class="pi pi-trash"></i>
          </k-button>
        </template>
        
        <!-- buttons to show when the model is not downloaded -->
        <template v-else>
          <k-button clear inline @click="onDownload(name)" :disabled="downloadingModel !== null" class="w-2/3">
            <i class="pi pi-download mr-2"> Download</i>
          </k-button>
        </template>
        
        <!-- expand/collapse button -->
        <k-button clear inline @click="toggleExpand(name)" class="w-1/3">
          <i :class="expandedModels.includes(name) ? 'pi pi-angle-up' : 'pi pi-angle-down'"></i>
        </k-button>
      </template>

    </k-list-item>
  </k-list>
  
  <Dialog :opened="showDeleteConfirm" title="Confirm Delete" @backdrop-click="showDeleteConfirm = false">
    <p class="mb-4">Are you sure you want to delete <strong>{{ modelToDelete }}</strong>?</p>
    <div class="flex gap-2">
      <k-button @click="showDeleteConfirm = false" outline>Cancel</k-button>
      <k-button @click="proceedDelete" class="k-color-brand-red">Delete</k-button>
    </div>
  </Dialog>
</template>