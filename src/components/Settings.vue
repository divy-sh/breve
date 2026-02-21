<script setup lang="ts">
  import { kPopup, Navbar, Page, Link } from 'konsta/vue';
  import ModelsDownload from './ModelsDownload.vue';
  import InferenceSettings from './InferenceSettings.vue';

  const { openSettings } = defineProps<{ openSettings: boolean }>();

  const emit = defineEmits<{
      (e: 'close'): void;
  }>();
</script>

<template>
    <k-popup :opened="openSettings" @backdropclick="emit('close')">
      <Page>
        <Navbar title="Settings">
          <template #right>
            <Link icon-only @click="emit('close')">
              <i class="pi pi-times p-2"></i>
            </Link>
          </template>
        </Navbar>

        <ModelsDownload/>
        <InferenceSettings/>
      </Page>

    </k-popup>
</template>

