<template>
  <span class="create-wrapper">

    <select v-model="serviceId" v-show="step === 1" @change="next" ref="step1">
      <option selected disabled :value="null">Service wählen...</option>
      <option v-for="service in services" :value="service.id" :key="service.id">
        {{ service.name }}
      </option>
    </select>

    <input placeholder="Zeit in Minuten..." v-show="step === 2" v-model="minutes" type="number" @keydown.enter="next" ref="step2" />

    <input placeholder="Notiz..." v-show="step === 3" v-model="note" @keydown.enter="saveEntry" ref="step3" />

    <check-icon @click="saveEntry" class="btn" />
    <x-icon @click="reset" class="btn" />
  </span>
</template>

<script setup>
import { ref, defineProps, inject, defineEmits } from 'vue';
import { XIcon, CheckIcon } from '@heroicons/vue/solid';
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps(['projectId']);
const emit = defineEmits(['close'])
const step = ref(1);

const note = ref('');
const serviceId = ref(null);
const minutes = ref(null);

const services = inject('services');

const step1 = ref(null);
const step2 = ref(null);
const step3 = ref(null);

function next() {
  step.value += 1;
  
  window.setTimeout(() => {
    if (step.value === 2) {
      step2.value.focus();
      return;
    }
    
    if (step.value === 3) {
      step3.value.focus();
    }
  }, 250);
}

function reset() {
  step.value = 1;
  note.value = '';
  serviceId.value = null;
  minutes.value = 0;

  emit('close');
}

function create(projectId, serviceId = null, minutes = 0, note = "") {
  invoke('create_time', { projectId, serviceId, minutes, note }).then((res) => {
    const data = JSON.parse(res);

    console.log(data);
    window.dispatchEvent(new CustomEvent('notify', { detail: 'Zeiteintrag erstellt' }));
  }).catch(err => console.log(err));
}

function saveEntry() {
  create(props.projectId, serviceId.value, minutes.value, note.value);

  reset();
}
</script>

<style scoped>
.create-wrapper {
  display: inline-flex;
  align-items: center;
  margin-left: 0.5rem;
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

select, input {
  border: none;
  border-bottom: 1px solid #ccc;
  outline: 0;
  padding: 0.25rem;
  font-size: 12px;
  -webkit-appearance: none;
}

select {
  box-shadow: 0;
  border-radius: 0;
  position: relative;
}

select option {
  font-size: 0.75rem;
}
</style>
