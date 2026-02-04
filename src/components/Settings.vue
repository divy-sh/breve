<script setup lang="ts">
    import {
        kPage,
        kNavbar,
        kPopup,
        kLink,
    } from 'konsta/vue';

    const props = defineProps<{
        openSettings: boolean;
    }>();

    const emit = defineEmits<{
        (e: 'close'): void;
    }>();
</script>

<template>
    <k-popup :opened="openSettings" @backdropclick="emit('close')">
      <k-page>
        <k-navbar title="Settings">
          <template #right>
            <k-link icon-only @click="emit('close')">
              <i class="pi pi-times p-2"></i>
            </k-link>
          </template>
        </k-navbar>
      </k-page>
    </k-popup>
</template>