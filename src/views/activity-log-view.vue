<template>
  <div>
    <h1>
      Activity Log
    </h1>

    <div>
      <input type="date" v-model="selectedDate" />
    </div>

    <div>
      <span v-if="entries.length === 0">No log found for the selected date</span>

      <div v-else>
        <div>
          <label>
            <input type="radio" v-model="mode" value="app" name="mode" />
            App
          </label>

          <label>
            <input type="radio" v-model="mode" value="window" name="mode" />
            Window
          </label>

          <label>
            <input type="radio" v-model="mode" value="log" name="mode" />
            Log
          </label>
        </div>

        <activity-table :entries="entries" :mode="mode"></activity-table>

        <activity-chart :entries="entries" v-show="mode === 'app'"></activity-chart>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { ActivityLog } from '../helper/activity-log';
import ActivityChart from '@/components/activity-log/activity-chart.vue';
import ActivityTable from '@/components/activity-log/activity-table.vue';

const selectedDate = ref('');

const entries = ref([]);

const mode = ref('app');

watch(selectedDate, (newSelect) => {

  ActivityLog.day(newSelect).then((res) => {
    entries.value = res;
  }).catch(() => {
    // on error set the value to empty array
    entries.value = [];
  });

});
</script>
