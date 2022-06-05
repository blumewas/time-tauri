<template>
  <a @click.prevent="scrollToTop" class="to-top-link" v-show="show">
    <arrow-up-icon class="to-top-icon" />
  </a>
</template>

<script setup>
import { ArrowUpIcon } from '@heroicons/vue/outline';
import { ref } from 'vue';

const show = ref(false);

function scrollToTop() {
  document.querySelector('main').scrollIntoView({
    block: "start",
    behavior: "smooth",
  });

  show.value = false;
}

document.addEventListener('scroll', () => {
  show.value = true;
});
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
