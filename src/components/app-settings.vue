<template>
  <div id="settings">
    <div v-show="showSettings" class="settings-container">
      <h3>Einstellungen</h3>

      <div>
        <input placeholder="Mite App..." type="text" v-model="miteApp" @keydown.enter="saveMiteApp" @blur="saveMiteApp" />
      </div>

      <div>
        <input placeholder="Api Key..." type="text" v-model="apiKey" @keydown.enter="saveApiKey" @blur="saveApiKey" />
      </div>
    </div>

    <cog-icon class="cog" @click="showSettings = !showSettings" />
  </div>
</template>

<script setup>
import { CogIcon } from '@heroicons/vue/outline';
import { AppSettings } from '../composeables/app-settings';
import { ref, defineEmits } from 'vue';

const emit = defineEmits(['updated', 'loaded']);

const appSettings = new AppSettings({
  apiKey: '',
  stared: [],
  miteApp: '',
});

const apiKey = ref('');
const miteApp = ref('');

appSettings.load().then((settings) => {
  miteApp.value = settings.miteApp;
  apiKey.value = settings.apiKey;

  emit('loaded', settings);
});

const showSettings = ref(false);

function saveApiKey() {
  appSettings.set('apiKey', apiKey.value)
    .then((settings) => emit('updated', settings));
}

function saveMiteApp() {
  appSettings.set('miteApp', miteApp.value)
    .then((settings) => emit('updated', settings));
}
</script>

<style scoped>
#settings {
  position: fixed;
  right: 0;
  bottom: 0;
  margin: 0.5rem;
  padding: 0.5rem;
  background-color: #fff;
}

.settings-container {
  box-shadow: 1px 1px 8px 1px rgba(0, 0, 0, 0.2);
  border-radius: 2px;
  padding: 1rem;
  background-color: #ffe;
  margin-bottom: .5rem;
}

.settings-container input {
  background-color: #ffe;
}

h3 {
  font-size: 0.75rem;
  margin: 0;
  margin-bottom: 0.25rem;
}

.cog {
  height: 2.5rem;
  width: 2.5rem;
  float: right;
}

.cog:hover, .to-top:hover {
  cursor: pointer;
  color: #ccc;
}
</style>
