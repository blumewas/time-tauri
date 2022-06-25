<template>
  <div>
    <h1>
      Projects
    </h1>

    <div class="filter">
      <h2 class="accordion-header" @click="showFilter = !showFilter">
        Filter
        <chevron-up-icon class="open-accordion" :class="{ 'rotate': showFilter }" />
      </h2>

      <div v-show="showFilter">
        <div>
          <input class="search" placeholder="Suche..." type="text" v-model="filter.search" @input="runFilter" />
        </div>

        <div>
          <label>
            <input type="checkbox" v-model="filter.hideUnstared" />
            Einträge ohne Stern ausblenden?
          </label>
        </div>
      </div>
    </div>

    <div v-if="filter.result">
      <mite-projects :customer-projects="filter.result" :stared="stared ?? []"
        v-show="!filter.hideUnstared" />
    </div>
    <div v-else>
      <mite-projects :customer-projects="staredProjects" :stared="true" />

      <hr v-show="!filter.hideUnstared" />

      <mite-projects :customer-projects="projects" :stared="stared ?? []" v-show="!filter.hideUnstared" />
    </div>
  </div>
</template>

<script setup>
import { ChevronUpIcon } from '@heroicons/vue/outline';

import MiteProjects from '../components/projects/mite-projects.vue';

import { ref, reactive, defineProps, computed } from 'vue';

const props = defineProps({
  projects: {
    type: Object,
    default:() => {},
  },
  stared: {
    type: Array,
    default:() => [],
  }
});

// filter hide unstared
const filter = reactive({
  hideUnstared: false,
  search: '',
  result: null,
});

// get stared projects
const staredProjects = computed(() => {
  return Object
    .entries(props.projects)
    .reduce((map, [customer, projects]) => {
      
      projects.forEach((project) => {

        const { id, name } = project;

        if (props.stared.includes(id)) {
          if (!map[customer]) {
            map[customer] = [];
          }

          map[customer].push({ id, name });
        }
      });
      
      return map;
    }, {});
});

const showFilter = ref(false);
function runFilter() {
  const {search} = filter;

  // clear result if string is empty
  if (!search || search === '') {
    filter.result = null;
    return;
  }

  // set our filter result if search has contents
  const s = search.toLocaleLowerCase();
  filter.result = Object.entries(props.projects).reduce((map, [customer, projects]) => {

    // return all projects if the customer includes our search string
    if (customer.toLowerCase().includes(s)) {
      map[customer] = projects;

      return map;
    }

    // filter projects
    const filteredProjects = projects.filter((p) => p.name.toLowerCase().includes(s));
    if (filteredProjects.length > 0) {
      // if we have projects that match our search we put it along customer in map
      map[customer] = filteredProjects;
    }

    return map;
  }, {});
}
</script>
