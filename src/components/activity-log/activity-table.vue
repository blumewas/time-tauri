<template>
  <div>
    <table>
      <thead>
        <tr>
          <th>App</th>
          <th>View</th>
          <th>Time</th>
        </tr>
      </thead>

      <tbody v-if="mode === 'app'">
        <tr v-for="(row, index) in rows" :key="index" >
          <td>{{ row.app }}</td>
          <td>{{ row.window }}</td>
          <td>{{ row.time }}</td>
        </tr>
      </tbody>

      <tbody v-else-if="mode === 'window'">
        <template v-for="[app, value] in Object.entries(rows)" :key="app" >
          <tr v-for="v in value" :key="v.window">
            <td>{{ app }}</td>
            <td>{{ v.window }}</td>
            <td>{{ dayjs.duration(v.time , 'seconds').format('HH:mm:ss') }}</td>
          </tr>
        </template>
      </tbody>

      <tbody v-else-if="mode === 'log'">
        <tr v-for="(row, index) in entryRows" :key="index" >
          <td>{{ row.app }}</td>
          <td>{{ row.window }}</td>
          <td>{{ dayjs.duration(row.time, 'seconds').format('mm:ss') }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ActivityLog } from '@/helper/activity-log';
import { computed, defineProps } from 'vue';

import duration from 'dayjs/plugin/duration';
import dayjs from 'dayjs';

dayjs.extend(duration);

const props = defineProps(['entries', 'mode']);

const entryRows = computed(() => {
  return props.entries.map((entry) => {
    const { app, window, time } = entry;
      const t = Number.parseInt(time.replace('s', ''), 10);

    return {
      app,
      window,
      time: t,
    };
  });
});

const rows = computed(() => {
  if (props.mode === 'app') {
    return Object.entries(ActivityLog.groupByApp(props.entries))
      .map(([app, value]) => {
        return {
          app,
          window: '-',
          time: dayjs.duration(value.total, 'seconds').format('HH:mm:ss'),
        }
      });
  }

  // group by app & view
  return entryRows.value.reduce((rows, entry) => {
    const { app, window, time } = entry;

      if (!rows[app]) {
        rows[app] = [];
      }

      // find the matching entry by window name
      const rowentry = rows[app].find(e => e.window === window);
      if (!rowentry) {
        rows[app].push({
          app,
          window,
          time,
        });

        return rows;
      }

      rowentry.time += time;

      return rows;
  }, {});
});
</script>

<style scoped>
table {
  width: 100%;
  margin: 2rem 0 2rem; 
  text-align: center;
}

tbody tr td {
  padding: 0.5rem;
}

th, td {
  border-bottom: 1px solid #ccc;
}
</style>
