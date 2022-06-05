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
            :checked="stared === true || stared.includes(project.id)"
            :show="displayProjectOptions === project.id"
            @star="triggerStar(project.id)"
            @unstar="triggerUnstar(project.id)"
          />

          {{ project.name }}
        </span>
        <create-entry @close="selectedProject = 0" :project-id="project.id" v-show="selectedProject === project.id" />

        <span v-show="selectedProject !== project.id" class="options">
          <pencil-icon @click="selectedProject = project.id" class="btn" />

          <start-timer :project-id="project.id" />
        </span>
      </div>
    </div>
  </div>

</template>

<script setup>
import { defineProps, ref } from 'vue';
import { PencilIcon } from '@heroicons/vue/solid';

import CreateEntry from './create-entry.vue';
import StarToggle from './star-toggle.vue';
import { trigger } from '@/composeables/emit';
import StartTimer from './start-timer.vue';

const props = defineProps(['stared', 'customerProjects']);

const selectedProject = ref(0);

const displayProjectOptions = ref(0);

function triggerStar(projectId) {
  trigger('star', projectId);
}

function triggerUnstar(projectId) {
  trigger('unstar', projectId);
}
</script>

<style scoped>
.customer {
  margin: 1.125rem 0;
  box-sizing: border-box;
}

.customer h2 {
  font-size: 1.125rem;
  font-weight: bold;
  margin-bottom: 1.125rem;
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
