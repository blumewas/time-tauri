<template>
  <!-- TODO add filter with search -->
  <!-- TODO display current time with stop button -->
  <main>
    <header ref="header">
      <h2 @click="showFilter = !showFilter">Filter</h2>
      
      <div v-show="showFilter">
        <div>
          <input class="search" placeholder="Suche..." type="text" v-model="search" @input="filter" />
        </div>

        <div>
          <label>
            <input type="checkbox" v-model="hideUnstared" />
            Einträge ohne Stern ausblenden?
          </label>
        </div>
      </div>
    </header>

    <div v-if="hasValidSettings">
      <mite-projects :customer-projects="staredCustomerProjects" @star="starProject" @unstar="unstarProject"
        :stared="true" />

      <hr v-show="!hideUnstared" />

      <mite-projects :customer-projects="customerProjects" @star="starProject" @unstar="unstarProject" :stared="false" v-show="!hideUnstared" />
    </div>

    <a @click.prevent="scrollToTop" class="to-top-link">
      <arrow-up-icon class="to-top" />
    </a>
  </main>

  <app-settings-component @updated="loadAll" @loaded="loadAll" />

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

import { computed, provide, ref } from 'vue';
import { ArrowUpIcon } from '@heroicons/vue/outline';

import MiteProjects from './components/mite-projects.vue';
import { AppSettings } from './composeables/app-settings';
import AppSettingsComponent from './components/app-settings.vue';

const appSettings = new AppSettings({
  apiKey: '',
  stared: [],
  miteApp: '',
});

const hideUnstared = ref(false);

const stared = ref([]);
const apiKey = ref('');
const miteApp = ref('');

const hasValidSettings = computed(() => apiKey.value !== '' && miteApp.value !== '');

function starProject(projectId) {
  stared.value.push(projectId);

  appSettings.set('stared', stared.value);

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
  if (!hasValidSettings.value) {
    return;
  }

  // load services from mite via our rust backend
  invoke('get_services').then((message) => {
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


function loadAll(settings) {
  if (settings) {
    stared.value = settings.stared;
    apiKey.value = settings.apiKey;
    miteApp.value = settings.miteApp;
  }

  loadProjects();
  loadServices();
}

const customerProjects = ref({});
const staredCustomerProjects = ref({});

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
  if (!hasValidSettings.value) {
    return;
  }

  // load the projects from mite via our rust backend
  invoke('get_projects').then((message) => {
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

const header = ref();
function scrollToTop() {
  header.value.scrollIntoView({
    block: "start",
    behavior: "smooth",
  });
}

const showFilter = ref(false);
const search = ref('');
function filter() {
  console.log(search.value);
}
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

.to-top-link {
  position: fixed;
  right: 0;
  top: 0;
  margin: 0.5rem;
  padding: 0.5rem;
}

.to-top {
  height: 1rem;
  width: 1rem;
  float: right;
}

input[type="text"] {
  box-sizing: border-box;
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

header {
  display: block;
  max-width: 300px;
  margin-bottom: 4rem;
}

header h2 {
  font-size: 1.25rem;
  font-weight: 100;
}

header div {
  margin: 0.5rem 0;
}

input[type=text].search {
  box-sizing: border-box;
  font-size: 1rem;
  padding: 0.5rem 0.75rem;
}
</style>
