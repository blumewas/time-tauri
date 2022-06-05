<template>
  <div id="notifications">
    <div class="notification-wrapper">
      <div class="notification" v-for="notification in notifications" :key="notification.id">
        {{ notification.text }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { useOn } from '../composeables/emit';
import { ref } from 'vue';

// notifications
const notifications = ref([]);

useOn('notify', (message) => {
  const id = new Date().getTime();

  notifications.value.push({
    id,
    text: message,
  });

  window.setTimeout(() => {
    notifications.value = notifications.value.filter(v => v.id !== id);
  }, 3000);
});
</script>

<style scoped>
#notifications {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 100;
  pointer-events: none;
}

.notification-wrapper {
  display: flex;
  flex-direction: column;
  padding: 0.5rem;
  float: right;
}

.notification {
  font-size: 0.875rem;
  border-radius: 0.25rem;
  box-shadow: 1px 1px 8px 1px rgba(0, 0, 0, 0.2);
  color: #456613;
  background-color: #bdff59;
  margin-top: 0.25rem;
  padding: 1rem;
}
</style>
