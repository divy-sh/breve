<template>
  <Page>
    <Navbar title="Downloading Model" />

    <Block class="text-center">
      <Block inset>
        <img
          src="../assets/icon.png"
          alt="Breve Icon"
          width="100"
          height="100"
          class="mx-auto m-4"
        />
        <p v-if="progress === null">
          0%
        </p>
        <p v-else>
          {{ progress.toFixed(1) }}%
        </p>

        <Progressbar
          :progress="progress ?? 0.4"
          class="mt-3"
        />
      </Block>
    </Block>
  </Page>
</template>

<script>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { listen } from '@tauri-apps/api/event';

import {
  Page,
  Navbar,
  Block,
  Progressbar,
} from 'konsta/vue';

export default {
  name: 'DownloadingModel',
  components: {
    Page,
    Navbar,
    Block,
    Progressbar,
  },
  setup() {
    const progress = ref(null);

    let unlistenProgress = null;
    let unlistenDownloading = null;

    onMounted(async () => {
      unlistenProgress = await listen('download-progress', (event) => {
        if (event?.payload != null) {
          progress.value = Number(event.payload);
        }
      });

      unlistenDownloading = await listen('downloading-model', (event) => {
        if (event?.payload === false) {
          progress.value = 100;
        }
      });
    });

    onBeforeUnmount(() => {
      if (unlistenProgress) unlistenProgress();
      if (unlistenDownloading) unlistenDownloading();
    });

    return {
      progress,
    };
  },
};
</script>
