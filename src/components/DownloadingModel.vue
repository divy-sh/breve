<template>
    <div class="downloading-model">
        <div class="spinner"></div>
        <p v-if="progress === null">Downloading model for the first time. Please wait...</p>
        <p v-else>Downloading model: {{ progress.toFixed(1) }}%</p>
    </div>
</template>

<script>
import { onMounted, onBeforeUnmount, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';

export default {
    name: 'DownloadingModel',
    setup() {
        const progress = ref(null);
    let unlistenProgress = null;
    let unlistenDownloading = null;

        onMounted(async () => {
            unlistenProgress = await listen('download-progress', (event) => {
                // event.payload is the percentage number
                if (event && event.payload !== undefined && event.payload !== null) {
                    progress.value = Number(event.payload);
                }
            });

            // Ensure we clear UI when downloading-model false is emitted
            unlistenDownloading = await listen('downloading-model', (event) => {
                if (event && event.payload === false) {
                    progress.value = 100;
                }
            });
        });

        onBeforeUnmount(() => {
            if (unlistenProgress) { unlistenProgress(); }
            if (unlistenDownloading) { unlistenDownloading(); }
        });

        return { progress };
    }
}
</script>

<style scoped>
.downloading-model {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    color: var(--text-color);
    align-items: center;
    justify-content: center;
    text-align: center;
}

.spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #ccc;
    border-top: 4px solid #42b983;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}
</style>