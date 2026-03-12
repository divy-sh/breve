<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useConversations } from '../composables/useConversations';
import { listen } from '@tauri-apps/api/event';
import { kListItem, kButton, kChip, kProgressbar } from 'konsta/vue';


const props = defineProps<{ model: any; modelName: string }>();

const { deleteModel, downloadModel, refreshVariables, onSubscribe, setDefaultModel, isSubscribed, defaultModel, downloadedModels } = useConversations();

const isDownloading = ref<boolean>(false);
const isExpanded = ref<boolean>(false);
const downloadProgress = ref(0);
const isSettingDefault = ref<string | null>(null);
const showDeleteConfirm = ref(false);
const modelToDelete = ref<string | null>(null);

const isDefault = (name: string) => defaultModel.value === name;

const isLocked = () => {
    return props.model['is_premium' as any] && !isSubscribed.value;
};

async function onDownload() {
    isDownloading.value = true;
    downloadProgress.value = 0;
    try {
        await downloadModel(props.modelName);
        await refreshVariables();
    } catch (err) {
        console.error("Download failed:", err);
    } finally {
        isDownloading.value = false;
        downloadProgress.value = 0;
    }
}

async function onSetDefault() {
    try {
        isSettingDefault.value = props.modelName;
        await setDefaultModel(props.modelName);
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


let unlistenProgress: (() => void) | null = null;
onMounted(async () => {
    unlistenProgress = await listen('download-progress', (event) => {
        downloadProgress.value = event.payload as number;
    });
});

onUnmounted(() => {
    unlistenProgress?.();
});
</script>

<template>
    <k-list-item>

        <template #title>
            {{ model['name' as any] }}
        </template>

        <template #after>
            {{ model['params' as any] }}
        </template>

        <template #subtitle>
            <div v-if="isDownloading">
                <div class="flex justify-between items-center mb-1">
                    <span>Downloading</span>
                    <span class="text-xs font-mono">{{ Math.round(downloadProgress) }}%</span>
                </div>
                <k-progressbar :progress="downloadProgress / 100" />
            </div>
            <div v-if="isExpanded">
                <k-chip class="m-0.5 max-w-full">
                    <span class="truncate">{{ model['repo' as any] }}</span>
                </k-chip>
                <k-chip class="m-0.5 max-w-full">
                    <span class="truncate">Size: {{ model['size' as any] }} MB</span>
                </k-chip>
                <k-chip v-if="model['is_thinking' as any] == true">
                    <span class="truncate">Thinking</span>
                </k-chip>
            </div>
            <template v-if="downloadedModels.includes(props.modelName)">
                <k-button v-if="isLocked()" clear inline @click="onSubscribe" class="w-1/3">
                    <i class="pi pi-star text-yellow-500 mr-2"> Subscribe</i>
                </k-button>
                <k-button v-else clear inline @click="onSetDefault()" :disabled="isDefault(props.modelName)" class="w-1/3">
                    <i v-if="isSettingDefault === props.modelName" class="pi pi-spinner pi-spin"></i>
                    <i v-else :class="isDefault(props.modelName) ? 'pi pi-check-circle' : 'pi pi-play-circle'"> Select</i>
                </k-button>
                <k-button clear inline @click="confirmDelete(props.modelName)" :disabled="isDefault(props.modelName)"
                    class="w-1/3">
                    <i class="pi pi-trash"> Delete</i>
                </k-button>
            </template>
            <template v-else>
                <k-button v-if="isLocked()" clear inline @click="onSubscribe" class="w-2/3">
                    <i class="pi pi-star text-yellow-500 mr-2"> Subscribe</i>
                </k-button>
                <k-button v-else clear inline @click="onDownload()" :disabled="isDownloading" class="w-2/3">
                    <i class="pi pi-download mr-2"> Download</i>
                </k-button>
            </template>
            <k-button clear inline @click="() => isExpanded = !isExpanded" class="w-1/3">
                <i :class="isExpanded ? 'pi pi-angle-up' : 'pi pi-angle-down'"> Details</i>
            </k-button>
        </template>
    </k-list-item>


    <Dialog :opened="showDeleteConfirm" title="Confirm Delete" @backdrop-click="showDeleteConfirm = false">
        <p class="mb-4">Are you sure you want to delete <strong>{{ modelToDelete }}</strong>?</p>
        <div class="flex gap-2">
            <k-button @click="showDeleteConfirm = false" outline>Cancel</k-button>
            <k-button @click="proceedDelete" class="k-color-brand-red">Delete</k-button>
        </div>
    </Dialog>
</template>