<template>  
  <main>
    <mite-projects :customer-projects="customerProjects" />
  </main>

  <div id="settings">
    <div v-show="showSettings">
      <input placeholder="Api Key..." v-model="apiKey" @keydown.enter="saveApiKey" @blur="saveApiKey" />
    </div>

    <cog-icon class="cog" @click="showSettings = !showSettings" />
  </div>

  <div id="notifications">
    <div class="notification-wrapper">
      <div class="notification" v-for="notification in notifications" :key="notification.id">
        {{ notification.text }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import {appDir} from '@tauri-apps/api/path';
import {writeFile, readTextFile} from '@tauri-apps/api/fs';

import { provide, ref } from 'vue';
import { CogIcon } from '@heroicons/vue/outline';

import MiteProjects from './components/mite-projects.vue';


const defaultSettings = {
  apiKey: null,    
  stared: [],
};

const showSettings = ref(false);
const staredProjects = ref([]);
const apiKey = ref('loading');

provide('apiKey', apiKey);

appDir().then(path => {
  readTextFile(`${path}settings.txt`)
    .then((contents) => {
      const loaded = JSON.parse(contents);

      staredProjects.value = loaded.stared;
      apiKey.value = loaded.apiKey;

      loadProjects();
      loadServices();
    })
    .catch(() => {
      // try to write default settings here
      const settingsJson = JSON.stringify(defaultSettings);

      writeFile({contents: settingsJson, path: `${path}settings.txt`});
    });
});

function saveApiKey() {
  loadProjects();
  loadServices();

  appDir().then(path => {
    const settingsJson = JSON.stringify({
      apiKey: apiKey.value,
      stared: staredProjects.value,
    });

    writeFile({contents: settingsJson, path: `${path}settings.txt`});
  });
}

const customerProjects = ref();
const services = ref();

provide('services', services);

function loadServices() {
  // load services from mite via our rust backend
  invoke('get_services', { apiKey: apiKey.value }).then((message) => {
    const data = JSON.parse(message);

    services.value = data.map((value) => {
      const {service} = value;
      return {
        id: service.id,
        name: service.name,
      };
    });
  });
}

function loadProjects() {
  // load the projects from mite via our rust backend
  invoke('get_projects', { apiKey: apiKey.value }).then((message) => {
    const data = JSON.parse(message);

    customerProjects.value =  data.reduce((customerProjectsMap, value) => {
      const { project } = value;
      if (project.archived) {
        return customerProjectsMap;
      }

      if (!customerProjectsMap[project.customer_name]) {
        customerProjectsMap[project.customer_name] = [];
      }

      customerProjectsMap[project.customer_name].push({
        id: project.id,
        name: project.name,
      });

      return customerProjectsMap;
    }, {});

  });
}

// notifications
const notifications = ref([]);

window.addEventListener('notify', (event) => {
  const { detail } = event;
  const id = new Date().getTime();

  notifications.value.push({
    id,
    text: detail,
  });

  window.setTimeout(() => {
    notifications.value = notifications.value.filter(v => v.id !== id);
  }, 3000);
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin-top: 60px;
}

main {
  display: block;
  margin: 0 auto;
  max-width: 600px;
}

#notifications {
  position: absolute;
  top: 0;
  left:0;
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
  padding: 1rem;
}

#settings {
  position: fixed;
  right: 0;
  bottom: 0;
  margin: 0.5rem;
}

.cog {
  height: 2.5rem;
  width: 2.5rem;
  float: right;
}

.cog:hover {
  cursor: pointer;
  color: #ccc;
}

input {
  border: none;
  border-bottom: 1px solid #ccc;
  outline: 0;
  padding: 0.25rem;
  font-size: 12px;
  -webkit-appearance: none;
}
</style>
