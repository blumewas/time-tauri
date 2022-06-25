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
        <activity-table :entries="entries"></activity-table>

        <activity-chart :entries="entries"></activity-chart>
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

watch(selectedDate, (newSelect) => {

  ActivityLog.day(newSelect).then((res) => {
    entries.value = res;
  }).catch(() => {
    // on error set the value to empty array
    entries.value = [];
  });

});
</script>
