<script setup lang="ts">
  import { 
    kPopup, Navbar, Page, Link, kBlockTitle, kBlock, 
    kList, kListItem, kRange, kButton, kListInput 
  } from 'konsta/vue';
  import { useSettings } from '../composables/useSettings';
  import { onMounted, ref } from 'vue';
  import { ModelConfig } from '../types';
  
  const { getModelConfig, setModelConfig } = useSettings();
  const { openSettings } = defineProps<{ openSettings: boolean }>();

  const modelSettings = ref<ModelConfig>({ temperature: 0, system_prompt: "", max_output_length: 0, max_context_length: 0 });

  const emit = defineEmits<{
      (e: 'close'): void;
  }>();

  onMounted(async () => {
    const response = await getModelConfig();
    modelSettings.value = (response as unknown) as ModelConfig;
  });

  const handleSave = async () => {
    await setModelConfig(modelSettings.value);
    emit('close');
  };
</script>

<template>
  <k-popup :opened="openSettings" @backdropclick="emit('close')">
    <Page>
      <Navbar title="Model Settings">
        <template #right>
          <Link navbar @click="emit('close')">Close</Link>
        </template>
      </Navbar>

      <k-block-title>Model Parameters</k-block-title>
      <k-list strong inset dividers>
        
        <k-list-item 
          label 
          title="Temperature" 
          :after="modelSettings.temperature.toFixed(2)"
        >
          <template #inner>
            <div class="flex items-center space-x-4 w-full mt-2">
              <span class="text-xs text-gray-500">0</span>
              <k-range
                :value="modelSettings.temperature * 100"
                :step="1"
                @input="(e: any) => (modelSettings.temperature = parseFloat(e.target.value) / 100.0)"
              />
              <span class="text-xs text-gray-500">1.0</span>
            </div>
          </template>
        </k-list-item>

        <k-list-item 
          label 
          title="Max Output Tokens" 
          :after="Math.round(modelSettings.max_output_length).toString()"
        >
          <template #inner>
            <div class="flex items-center space-x-4 w-full mt-2">
              <span class="text-xs text-gray-500">1k</span>
              <k-range
                :value="(modelSettings.max_output_length - 1024) / 3072 * 100"
                @input="(e: any) => (modelSettings.max_output_length = Math.round(parseFloat(e.target.value) / 100.0 * 3072 + 1024))"
              />
              <span class="text-xs text-gray-500">4k</span>
            </div>
          </template>
        </k-list-item>

        <k-list-item 
          label 
          title="Context Window" 
          :after="Math.round(modelSettings.max_context_length).toString()"
        >
          <template #inner>
            <div class="flex items-center space-x-4 w-full mt-2">
              <span class="text-xs text-gray-500">4k</span>
              <k-range
                :value="(modelSettings.max_context_length - 4096) / 28672 * 100"
                @input="(e: any) => (modelSettings.max_context_length = Math.round(parseFloat(e.target.value) / 100 * 28672 + 4096))"
              />
              <span class="text-xs text-gray-500">32k</span>
            </div>
          </template>
        </k-list-item>
      </k-list>

      <k-block-title>Personalization</k-block-title>
      <k-list strong inset dividers>
        <k-list-input
          title="System Prompt"
          type="textarea"
          placeholder="e.g. You are a helpful assistant..."
          :value="modelSettings.system_prompt"
          @input="(e: any) => (modelSettings.system_prompt = e.target.value)"
          resizable
          input-class="!h-32 text-sm" 
        />
      </k-list>

      <k-block class="space-y-2">
        <k-button large rounded @click="handleSave" class="shadow-md">
          <i class="pi pi-save mr-2"></i> Save
        </k-button>
        <k-button large rounded outline @click="emit('close')">
          Cancel
        </k-button>
      </k-block>
    </Page>
  </k-popup>
</template>