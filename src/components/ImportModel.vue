<script setup lang="ts">
    import { kBlockTitle, kList } from 'konsta/vue';

    const { openImportModel } = defineProps<{ openImportModel: boolean }>();
    const emit = defineEmits<{
        (e: 'close'): void;
    }>();
</script>

<template>
    <k-popup :opened="openImportModel" @backdropclick="emit('close')">
        <k-block-title>Import Model</k-block-title>
        <k-list strong inset dividers>
            <p class="p-4 text-center text-gray-500">Model import functionality is not implemented yet.</p>
        </k-list>
    </k-popup>
</template>