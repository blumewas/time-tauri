<template>
  <div>
    <div>
      <label>
        <input type="radio" v-model="mode" value="app" name="mode" />
        App
      </label>

      <label>
        <input type="radio" v-model="mode" value="window" name="mode" />
        Window
      </label>
    </div>
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
          <td>{{ row.view }}</td>
          <td>{{ row.time }}</td>
        </tr>
      </tbody>

      <tbody v-else>
        <template v-for="[app, value] in Object.entries(rows)" :key="app" >
          <tr v-for="v in value" :key="v.window">
            <td>{{ app }}</td>
            <td>{{ v.window }}</td>
            <td>{{ dayjs.duration(v.time , 'seconds').format('HH:mm:ss') }}</td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ActivityLog } from '@/helper/activity-log';
import { ref, computed, defineProps } from 'vue';

import duration from 'dayjs/plugin/duration';
import dayjs from 'dayjs';

dayjs.extend(duration);

const props = defineProps(['entries']);

const mode = ref('app');

const rows = computed(() => {
  if (mode.value === 'app') {
    return Object.entries(ActivityLog.groupByApp(props.entries))
      .map(([app, value]) => {
        return {
          app,
          view: '-',
          time: dayjs.duration(value.total, 'seconds').format('HH:mm:ss'),
        }
      });
  }

  // group by app & view
  return props.entries.reduce((rows, entry) => {
    const { app, window, time } = entry;
      const t = Number.parseInt(time.replace('s', ''), 10);

      if (!rows[app]) {
        rows[app] = [];
      }

      // find the matching entry by window name
      const rowentry = rows[app].find(e => e.window === window);
      if (!rowentry) {
        rows[app].push({
          app,
          window,
          time: t,
        });

        return rows;
      }

      rowentry.time += t;

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
