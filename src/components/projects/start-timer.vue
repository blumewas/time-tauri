<template>
  <div class="play-btn">
    <play-icon @click="start" @contextmenu.prevent="startRight" class="btn" />

    <ul class="dropdown" v-show="showServiceDropdown">

      <li v-for="service in services" :key="service.id" class="option" @click="start($event, service.id)">
        {{ service.name }}
      </li>
    
    </ul>
  </div>
</template>

<script setup>
import { Mite } from '@/commands/mite';
import { trigger, useOn } from '@/composeables/emit';
import { defineProps, inject, ref } from 'vue';
import { PlayIcon } from '@heroicons/vue/solid';

const props = defineProps(['projectId']);

const services = inject('services');

const showServiceDropdown = ref(false);

function start(event, serviceId = null) {
  // check for ctrl on mac
  if (event.ctrlKey) {
    return;
  }

  Mite.startTimer(props.projectId, serviceId)
    .then(({ tracker }) => {
      trigger('notify', 'Timer started');

      showServiceDropdown.value = false;

      trigger('started-timer', tracker);
    })
    .catch(err => console.log(err));
}

function startRight() {
  showServiceDropdown.value = true;
}

useOn('clicked-document', (event) => {
  if (!event.target.classList.contains('option')) {
    showServiceDropdown.value = false;
  }
});
</script>

<style scoped>
.play-btn {
  position: relative;
  display: flex;
  align-items: center;
}

.dropdown {
  position: absolute;
  list-style: none;
  max-width: 10rem;
  border: 1px solid #ccc;
  padding: 0;
  margin-block: 0;
  z-index: 99;
  background-color: #fff;
  overflow-y: scroll;
  max-height: 12rem;
  right: 0.25rem;
  top: 1.75rem;
}

.option {
  font-size: 0.875rem;
  padding: 0.25rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
}

.option:hover {
  background-color: #ccc;
}
</style>
