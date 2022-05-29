<template>
  <div class="star-wrapper">
    <span class="star-option" v-show="props.checked || props.show">
      <star-outline class="star" v-show="!props.checked" @click="$emit('star')" />
      <star-icon class="star" v-show="props.checked" @click="$emit('unstar')" />
    </span>
  </div>
</template>

<script setup>
import { StarIcon as StarOutline } from '@heroicons/vue/outline';
import { StarIcon } from '@heroicons/vue/solid';

import { defineProps } from 'vue';

const props = defineProps(['checked', 'show']);
</script>

<style scoped>

.star {
  height: 1.5rem;
  width: 1.5rem;
}

.star:hover {
  color: #ccc;
  cursor: pointer;
}

.star-wrapper {
  margin-right: 1rem;
  width: 1.5rem;
  min-width: 1.5rem;
}

.star-option {
  display: flex;
  align-items: center;
}
</style>
