<template>
  <div>
    <div v-if="Object.keys(props.customerProjects).length === 0">
      Keine Einträge vorhanden
    </div>

    <div v-else v-for="(projects, customer) in props.customerProjects" :key="customer" class="customer">
      <h2>{{ customer }}</h2>

      <div v-for="project in projects" :key="project.id" class="project">
        <span class="project-title"
          @mouseenter="displayProjectOptions = project.id"
          @mouseleave="displayProjectOptions = 0"
        >
          <star-toggle 
            :checked="stared"
            :show="displayProjectOptions === project.id"
            @star="$emit('star', project.id)"
            @unstar="$emit('unstar', project.id)"
          />

          {{ project.name }}
        </span>
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
import { defineProps, ref } from 'vue';
import { PlayIcon, PencilIcon } from '@heroicons/vue/solid';


import { invoke } from '@tauri-apps/api/tauri';
import CreateEntry from './create-entry.vue';
import StarToggle from './star-toggle.vue';

const props = defineProps(['stared', 'customerProjects']);

const selectedProject = ref(0);

const displayProjectOptions = ref(0);

function start(projectId) {

  invoke('create_time', { projectId }).then((res) => {
    const data = JSON.parse(res);
    const entryId = data.time_entry.id;

    invoke('start_stop_time', { entryId }).then(() => {
    
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
  margin-bottom: 1.25rem;
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

.options {
  display: inline-flex;
  align-items: center;
  margin-left: 0.5rem;
}

.project-title {
  display: inline-flex;
  align-items: center;
  vertical-align: middle;
  cursor: pointer;
}
</style>
