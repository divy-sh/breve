<script setup lang="ts">
  import { kPopup, Navbar, Page, Link, kBlockTitle, kBlock, kList, kListItem, kRange, kButton } from 'konsta/vue';
  import { ref } from 'vue';

  const { openSettings: openSettings } = defineProps<{ openSettings: boolean }>();

  const emit = defineEmits<{
      (e: 'close'): void;
  }>();

  // TODO set and get this from backend (needs to be implemented first)
  const temperature = ref(0.5);
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

        <k-block-title>Temperature {{ temperature}}</k-block-title>
        <k-block>
          <k-list strong inset>
            <k-list-item inner-class="flex gap-4 items-center">
              <template #inner>
                <span>0</span>
                <k-range
                        :value="temperature * 100"
                        :step="1"
                        @input="(e: any) => (temperature = parseFloat(e.target.value) / 100.0)"
                      />
                <span>1.0</span>
              </template>
            </k-list-item>
            <k-list-item>
              <k-button><i class="pi pi-save"></i>  Save</k-button>
            </k-list-item>
          </k-list>
        </k-block>
      </Page>
    </k-popup>
</template>
