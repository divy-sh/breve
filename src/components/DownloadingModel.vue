<template>
    <div class="downloading-model">
        <div class="breve-icon">
            <img src="../assets/icon.png" alt="Breve Icon" width="100" height="100" />
        </div>
        <div v-if="progress === null">
            <p>Downloading model: {{ 0 }}%</p>
            <div class="progress-bar">
                <div class="progress-fill" :style="{ width: 0 + '%' }"></div>
            </div>        
        </div>
        <div v-else class="progress-container">
            <p>Downloading model: {{ progress.toFixed(1) }}%</p>
            <div class="progress-bar">
                <div class="progress-fill" :style="{ width: progress + '%' }"></div>
            </div>
        </div>
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
                if (event && event.payload !== undefined && event.payload !== null) {
                    progress.value = Number(event.payload);
                }
            });

            unlistenDownloading = await listen('downloading-model', (event) => {
                if (event && event.payload === false) {
                    progress.value = 100;
                }
            });
        });

        onBeforeUnmount(() => {
            if (unlistenProgress) unlistenProgress();
            if (unlistenDownloading) unlistenDownloading();
        });

        return { progress };
    }
}
</script>

<style scoped>
.breve-icon {
    margin-bottom: 2rem;
}

.downloading-model {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    color: var(--secondary-color);
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 1rem;
}

.progress-container {
    width: 80%;
    max-width: 400px;
}

.progress-bar {
    width: 100%;
    height: 10px;
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 9999px;
    overflow: hidden;
    margin-top: 0.5rem;
}

.progress-fill {
    height: 100%;
    background-color: var(--accent-color, #4caf50);
    width: 0%;
    transition: width 0.2s ease;
    border-radius: 9999px;
}
</style>
