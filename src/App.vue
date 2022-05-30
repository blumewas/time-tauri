<template>
  <!-- TODO add filter with search -->
  <!-- TODO display current time with stop button -->
  <main>
    <mite-projects :customer-projects="staredCustomerProjects" @star="starProject" @unstar="unstarProject"
      :stared="true" />

    <hr v-show="!hideUnstared" />

    <mite-projects :customer-projects="customerProjects" @star="starProject" @unstar="unstarProject" :stared="false" v-show="!hideUnstared" />

  <!-- TODO to top buton -->
  </main>

  <div id="settings">
    <!-- TODO extra component -->
    <!-- TODO add mite-app name -->
    <!-- TODO better background/highlight -->
    <div v-show="showSettings">
  
      <!-- TODO move to filter -->
      <div>
        <label>
          <input type="checkbox" v-model="hideUnstared" />
          Einträge ohne Stern ausblenden?
        </label>
      </div>

      <div>
        <input placeholder="Api Key..." type="text" v-model="apiKey" @keydown.enter="saveApiKey" @blur="saveApiKey" />
      </div>
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

import { provide, ref } from 'vue';
import { CogIcon } from '@heroicons/vue/outline';

import MiteProjects from './components/mite-projects.vue';
import { AppSettings } from './composeables/app-settings';

const appSettings = new AppSettings({
  apiKey: null,
  stared: [],
});

const showSettings = ref(false);
const hideUnstared = ref(false);

const stared = ref([]);
const apiKey = ref('loading');

provide('apiKey', apiKey);

appSettings.load().then((settings) => {
  stared.value = settings.stared;
  apiKey.value = settings.apiKey;
}).then(() => {
  loadProjects();
  loadServices();
});

function saveApiKey() {
  loadProjects();
  loadServices();

  appSettings.apiKey = apiKey.value;
  appSettings.save();
}

function starProject(projectId) {
  stared.value.push(projectId);

  appSettings.stared = stared.value;
  appSettings.save();
  loadProjects();
}

function unstarProject(projectId) {
  stared.value = stared.value.filter(val => val !== projectId);

  appSettings.stared = stared.value;
  appSettings.save();
  loadProjects();
}

// provide services
const services = ref();

provide('services', services);
function loadServices() {
  // load services from mite via our rust backend
  invoke('get_services', { apiKey: apiKey.value }).then((message) => {
    const data = JSON.parse(message);

    services.value = data.map((value) => {
      const { service } = value;
      return {
        id: service.id,
        name: service.name,
      };
    });
  });
}

const customerProjects = ref();
const staredCustomerProjects = ref();

function groupProjects(projects) {
  const staredCustomers = {};
  const customers = {};

  projects.forEach((value) => {
    const { project } = value;

    if (project.archived) {
      return;
    }

    const { id, customer_name, name } = project;

    if (stared.value.includes(id)) {

      if (!staredCustomers[customer_name]) {
        staredCustomers[customer_name] = [];
      }

      staredCustomers[customer_name].push({
        id,
        name,
      });

      console.log(staredCustomers);
      return;
    }

    if (!customers[customer_name]) {
      customers[customer_name] = [];
    }

    customers[customer_name].push({
      id,
      name,
    });
  });

  customerProjects.value = customers;
  staredCustomerProjects.value = staredCustomers;
}

function loadProjects() {
  // load the projects from mite via our rust backend
  invoke('get_projects', { apiKey: apiKey.value }).then((message) => {
    const data = JSON.parse(message);

    groupProjects(data);
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
  padding: 1rem;
}

#settings {
  position: fixed;
  right: 0;
  bottom: 0;
  margin: 0.5rem;
  padding: 0.5rem;
  background-color: #fff;
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

input[type="text"] {
  border: none;
  border-bottom: 1px solid #ccc;
  outline: 0;
  padding: 0.25rem;
  font-size: 12px;
  width: 100%;
}

hr {
  margin: 2rem 0;
}
</style>
