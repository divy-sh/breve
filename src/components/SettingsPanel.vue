<template>
  <div class="settings-overlay" @click.self="emit('close')">
    <div class="settings-panel">
      <div class="settings-header">
        <h3>Settings</h3>
        <button class="close-btn" @click="emit('close')">×</button>
      </div>
      <div class="settings-content">
        <p>Here you can manage your preferences.</p>
        <!-- Add actual settings inputs here -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const emit = defineEmits<{
  (e: 'close'): void;
}>();
</script>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.4);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 20;
}

.settings-panel {
  background-color: var(--background-color);
  padding: 2rem;
  border-radius: 1rem;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  color: var(--text-color);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--text-color);
  cursor: pointer;
}

.settings-content {
  color: var(--text-color);
}
</style>
