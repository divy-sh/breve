<script setup lang="ts">

    import { useConversations } from "../composables/useConversations";
    import {
      Page,
      Navbar,
      Block,
      kButton
    } from 'konsta/vue';

    const { downloadModel } = useConversations();

</script>

<template>
  <Page>
    <Navbar title="Download Model" />

    <Block class="text-center">
      <Block inset>
        <img
          src="../assets/icon.png"
          alt="Breve Icon"
          width="100"
          height="100"
          class="mx-auto m-4"
        />
        <h2>Download assets?</h2>
        <p>(Only required on first launch.)</p>
        <k-button @click="downloadModel" class="mt-4">Download</k-button>
      </Block>
    </Block>
  </Page>
</template>