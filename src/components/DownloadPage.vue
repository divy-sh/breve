<script setup lang="ts">

import { useConversations } from "../composables/useConversations";

const { downloadModel } = useConversations();

</script>

<template>
    <div class="download-page">
        <img src="../assets/icon.png" alt="Breve Icon" width="100" height="100" />
        <div class="header">
            <h2>Download assets?</h2>
            <p>(Only required on first launch.)</p>
        </div>
        <div class="input-area">
            <button type="submit" @click="downloadModel">Download</button>
        </div>
    </div>
</template>

<style scoped>
    .download-page {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        height: 100%;
        width: 100%;
        gap: 1rem;
        padding: 1rem;
        background-color: var(--background-color);
        color: var(--text-color);
    }

    .input-area button {
        margin-top: 1rem;
        padding: 1rem;
        background-color: var(--primary-color);
        color: var(--text-color);
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        font-weight: 600;
        transition: background-color 0.2s;
    }

    .input-area button:hover:not(:disabled) {
        background-color: var(--secondary-color);
    }

    .header {
        color: var(--secondary-color);
    }
</style>