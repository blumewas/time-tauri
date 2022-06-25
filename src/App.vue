<template>
  <main>
    <!-- TODO help view -->
    <div v-if="!hasValidSettings" class="center">
      <span>Please configure the App</span>
    </div>
    <div v-else-if="errorMsg" class="error center">
      {{ errorMsg }}
    </div>
    <div v-else>

      <app-timer />

      <nav>
        <a @click.prevent="view = 'projects'" :class="{'active': view === 'projects'}">Projects</a>

        <a @click.prevent="view = 'activity'" :class="{'active': view === 'activity'}">Activity</a>
      </nav>

      <projects-view v-show="view === 'projects'" :projects="projects" :stared="appSettings.stared" />

      <activity-log-view v-show="view === 'activity'"></activity-log-view>
    </div>

    <scroll-top-button />
  </main>

  <app-settings-component @updated="load" @loaded="load" />

  <app-notifications />
</template>

<script setup>
import { computed, provide, ref, reactive } from 'vue';

import ProjectsView from './views/projects-view.vue';

import AppTimer from './components/app-timer.vue';
import AppSettingsComponent from './components/app-settings.vue';
import ScrollTopButton from './components/scroll-top-button.vue';
import AppNotifications from './components/app-notifications.vue';

import { Mite } from './commands/mite';
import { Util } from './helper/util';
import { trigger } from './composeables/emit';
import ActivityLogView from './views/activity-log-view.vue';


// errorMessage
const errorMsg = ref(false);

// app view to display
const view = ref ('projects');

// our apps settings loaded from app folder
const appSettings = reactive({});

// compute if settings are valid
const hasValidSettings = computed(() => appSettings.apiKey !== '' && appSettings.miteApp !== '');

// provide services
const services = ref([]);
provide('services', services);

// reactive projects
const projects = ref([]);

/**
 * Load all data when we load our settings
 */
function load(settings) {
  errorMsg.value = false;
  
  if (settings) {
    appSettings.stared = settings.stared;
    appSettings.apiKey = settings.apiKey;
    appSettings.miteApp = settings.miteApp;
  }

  // only load if we have valid settings
  if (!hasValidSettings.value) {
    return;
  }

  // load all values we need from our project
  Promise.all([
    Mite.getProjects(),
    Mite.getServices()
  ])
  .then((results) => {
    const [_projects, _services] = results;

    projects.value = Util.groupBy(_projects, 'customer_name');
    services.value = _services;
  })
  .catch((error) => {
    errorMsg.value = error;
  });
  
}

document.addEventListener('click', (event) => {
  trigger('clicked-document', event);
});
</script>

<style>
* {
  padding: 0;
  margin: 0;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin-top: 2.5rem;
}

main {
  display: block;
  margin: 0 auto;
  max-width: 600px;
}

.open-accordion {
  height: 1rem;
  width: 1rem;
}

.open-accordion {
  margin-left: 0.5rem;
}

.rotate {
  transform: rotate(180deg);
}

.accordion-header {
  display: flex;
  align-items: center;
  cursor: pointer;
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

h1 {
  font-size: 1.5rem;
  margin: 0;
}

.filter {
  display: block;
  max-width: 300px;
  margin-bottom: 2rem;
}

.filter h2 {
  font-size: 0.875rem;
  font-weight: 100;
  margin: 0;
}

.filter div {
  margin: 0.25rem 0;
}

input[type=text].search {
  box-sizing: border-box;
  font-size: 1rem;
  padding: 0.5rem 0.25rem;
}

.error {
  color: red;
}

.center {
  text-align: center;
}

.btn {
  height: 1.5rem;
  width: 1.5rem;
  margin-left: 0.5rem;
}

.btn:hover {
  color: #ccc;
  cursor: pointer;
}

.btn[disabled] {
  color: #ccc;
  pointer-events: none;
}

nav {
  font-size: 1.125rem;
  font-weight: bold;
  margin-bottom: 1.5rem;
  display: flex;
  justify-content: center;
  gap: 4rem;
}

nav a:hover {
  cursor: pointer;
  border-bottom: 1px solid #2c3e50;
}

nav a.active {
  border-bottom: 1px solid #2c3e50;
}
</style>
