<template>
  <div>
    <div v-for="(projects, customer) in props.customerProjects" :key="customer" class="customer">
      <h2>{{ customer }}</h2>

      <div v-for="project in projects" :key="project.id" class="project">
        <span>{{ project.name }}</span>
        <create-entry @close="selectedProject = 0" :project-id="project.id" v-show="selectedProject === project.id" />

        <span v-show="selectedProject !== project.id" class="options">
          <pencil-icon @click="selectedProject = project.id" class="btn" />
          <play-icon @click="start(project.id)" class="btn" />
        </span>
      </div>
    </div>
  </div>

</template>

<script setup>
import { defineProps, inject, ref } from 'vue';
import { PlayIcon, PencilIcon } from '@heroicons/vue/solid';
import { invoke } from '@tauri-apps/api/tauri';
import CreateEntry from './create-entry.vue';

const props = defineProps(['customerProjects']);

const selectedProject = ref(0);
const apiKey = inject('apiKey');

function start(projectId) {

  invoke('create_time', { apiKey: apiKey.value, projectId }).then((res) => {
    const data = JSON.parse(res);
    const entryId = data.time_entry.id;

    invoke('start_stop_time', { apiKey: apiKey.value, entryId }).then(() => {
    
      window.dispatchEvent(new CustomEvent('notify', { detail: 'Timer gestartet' }));
      
    });
  }).catch(err => console.log(err));
}
</script>

<style scoped>
.customer {
  margin: 0.5rem 0;
}

.customer h2 {
  font-size: 1.25rem;
  font-weight: bold;
}

.project {
  display: flex;
  height: 1.5rem;
  align-items: center;
  margin: 0.25rem 1rem;
}

.btn {
  height: 1.5rem;
  width: 1.5rem;
  margin-left: 0.5rem;
}

.btns {
  display: inline-flex;
  align-items: center;
}

.btn:hover {
  color: #ccc;
  cursor: pointer;
}
</style>
