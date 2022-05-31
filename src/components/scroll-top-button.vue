<template>
  <a @click.prevent="scrollToTop" class="to-top-link">
    <arrow-up-icon class="to-top-icon" />
  </a>
</template>

<script setup>
import { ArrowUpIcon } from '@heroicons/vue/outline';

function scrollToTop() {
  document.querySelector('header').scrollIntoView({
    block: "start",
    behavior: "smooth",
  });
}
</script>

<style scoped>
.to-top-link {
  position: fixed;
  right: 0;
  top: 0;
  margin: 0.5rem;
  padding: 0.5rem;
  cursor: pointer;
}

.to-top-icon {
  height: 1rem;
  width: 1rem;
}
</style>
