<script setup lang="ts">
  import { kPopup, Navbar, Page, Link } from 'konsta/vue';
  import ModelList from './ModelList.vue';

  const { openModels: openModels } = defineProps<{ openModels: boolean }>();

  const emit = defineEmits<{
      (e: 'close'): void;
  }>();
</script>

<template>
    <k-popup :opened="openModels" @backdropclick="emit('close')">
      <Page>
        <Navbar title="Models">
          <template #right>
            <Link icon-only @click="emit('close')">
              <i class="pi pi-times p-2"></i>
            </Link>
          </template>
        </Navbar>

        <ModelList/>
      </Page>

    </k-popup>
</template>

