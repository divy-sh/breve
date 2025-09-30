<template>
    <div class="downloading-model">
        <div class="spinner"></div>
        <p>Downloading model for the first time. Please wait...</p>
    </div>
</template>

<script>
export default {
    name: 'DownloadingModel'
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