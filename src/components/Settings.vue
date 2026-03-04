<script setup lang="ts">
  import { kPopup, Navbar, Page, Link, kBlockTitle, kBlock, kList, kListItem, kRange, kButton, kListInput } from 'konsta/vue';
  import { useSettings } from '../composables/useSettings';
  import { onMounted, ref } from 'vue';
  import { ModelConfig } from '../types';
  
  const { getModelConfig } = useSettings();
  const { openSettings } = defineProps<{ openSettings: boolean }>();

  const modelSettings = ref<ModelConfig>({ temperature: 0, system_prompt: "" });

  const emit = defineEmits<{
      (e: 'close'): void;
  }>();

  onMounted(async () => {
    const response = await getModelConfig();
    modelSettings.value = (response as unknown) as ModelConfig;
  });
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

        <k-block-title>Temperature {{ modelSettings.temperature }}</k-block-title>
        <k-block>
          <k-list strong inset>
            <k-list-item inner-class="flex gap-4 items-center">
              <template #inner>
                <span>0</span>
                <k-range
                  :value="modelSettings.temperature * 100"
                  :step="1"
                  @input="(e: any) => (modelSettings.temperature = parseFloat(e.target.value) / 100.0)"
                />
                <span>1.0</span>
              </template>
            </k-list-item>
          </k-list>
        </k-block>
        
        <k-block-title>System Prompt</k-block-title>
        <k-block>
          <k-list strong inset>
            <k-list-input
              label="Instructions"
              type="textarea"
              placeholder="e.g. You are a helpful assistant..."
              :value="modelSettings.system_prompt"
              @input="(e: any) => (modelSettings.system_prompt = e.target.value)"
              resizable
              input-class="!h-32" 
            />
          </k-list>
        </k-block>

        <k-block>
          <k-list-item>
            <k-button><i class="pi pi-save"></i> Save</k-button>
          </k-list-item>
        </k-block>
      </Page>
    </k-popup>
</template>