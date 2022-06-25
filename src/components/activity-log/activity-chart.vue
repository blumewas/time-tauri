<template>
  <div>
    <Pie :chart-data="chartData" :chart-options="chartOptions" />
  </div>
</template>

<script setup>
import { computed, defineProps } from 'vue';
import { Pie } from 'vue-chartjs';
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  ArcElement,
  CategoryScale,
} from 'chart.js';
import duration from 'dayjs/plugin/duration';
import dayjs from 'dayjs';
import { ActivityLog } from '@/helper/activity-log';

dayjs.extend(duration);

ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

const props = defineProps(['entries']);

const chartOptions = {
  plugins: {
    tooltip: {
      callbacks: {
        label(l) {
          const { label, raw } = l;
          
          const format = dayjs.duration(raw, 'seconds').format('HH:mm:ss');
          return `${label} - ${format}`;
        }
      },
    },
  },
};

const chartData = computed(() => {

  const labels = [];
  const cdata = {
    backgroundColor: ['#41B883', '#E46651', '#00D8FF', '#DD1B16'],
    data: [],
  };

  const apps = ActivityLog.groupByApp(props.entries);

  for (const [app, value] of Object.entries(apps)) {
    labels.push(app);

    cdata.data.push(value.total);
  }

  return {
    labels,
    datasets: [
      cdata,
    ],
  };
});

</script>
