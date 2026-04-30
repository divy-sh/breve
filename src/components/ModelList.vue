<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { kBlockTitle, kList, kFab } from 'konsta/vue';
import { useConversations } from '../composables/useConversations';
import ModelCard from './ModelCard.vue';
import ImportModel from './ImportModel.vue';

const {
  availableModels,
  downloadedModels,
  refreshVariables,
  checkSubscription
} = useConversations();

// NEW: Computed properties for separated and sorted lists
const downloadedList = computed(() => {
  return Object.keys(availableModels.value)
    .filter(name => downloadedModels.value.includes(name))
    .sort((a, b) => {
      const contextA = availableModels.value[a]['size' as any] as any || 0;
      const contextB = availableModels.value[b]['size' as any] as any || 0;
      if (contextB !== contextA) return contextA - contextB;
      return b.localeCompare(a);
    });
});

const availableList = computed(() => {
  return Object.keys(availableModels.value)
    .filter(name => !downloadedModels.value.includes(name))
    .sort((a, b) => {
      const contextA = availableModels.value[a]['size' as any] as any || 0;
      const contextB = availableModels.value[b]['size' as any] as any || 0;
      if (contextB !== contextA) return contextA - contextB;
      return b.localeCompare(a);
    });
});


onMounted(async () => {
  await checkSubscription()
  await refreshVariables();
});
</script>

<template>
  <k-fab
      class="fixed right-safe-4 bottom-safe-4 z-20"
      text="Import Model"
      text-position="after"
      ><template #icon>
        <i class="pi pi-plus"></i>
      </template>
  </k-fab>
  <template v-if="downloadedList.length > 0">
    <k-block-title>Downloaded Models</k-block-title>
    <k-list strong inset dividers>
      <ModelCard v-for="name in downloadedList" :key="name" :model="availableModels[name]" :modelName="name"/>
    </k-list>
  </template>
  <template v-if="availableList.length > 0">
    <k-block-title >Available Models</k-block-title>
    <k-list strong inset dividers>
      <ModelCard v-for="name in availableList" :key="name" :model="availableModels[name]" :modelName="name"/>
    </k-list>
  </template>
  <ImportModel :openImportModel="false" @close="() => {}" />
</template>