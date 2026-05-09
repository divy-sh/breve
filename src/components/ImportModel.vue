<script setup lang="ts">
    import { kList, kPopup, kNavbar, Link } from 'konsta/vue';

    const { openImportModel } = defineProps<{ openImportModel: boolean }>();
    const emit = defineEmits<{
        (e: 'close'): void;
    }>();
</script>

<template>
    <k-popup :opened="openImportModel" @backdropclick="emit('close')">
        <k-navbar title="Import Model">
          <template #right>
            <Link icon-only @click="emit('close')">
              <i class="pi pi-times p-2"></i>
            </Link>
          </template>
        </k-navbar>
        <k-list strong inset dividers>
            <p class="p-4 text-center text-gray-500">Model import functionality is not implemented yet.</p>
        </k-list>
    </k-popup>
</template>