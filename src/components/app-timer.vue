<template>
  <div class="timer-wrapper">
    <div class="timer">
      <span v-text="hours"></span>

      <span>:</span>

      <span v-text="minutes"></span>
    </div>

    <div class="start-stop-btn">
      <a @click="start" class="btn">
        <play-icon class="btn" />
      </a>
      <a :disabled="isRunning ? null : true" @click="stop" class="btn">
        <stop-icon class="btn" />
      </a>
    </div>
  </div>
</template>

<script setup>
import { Mite } from '@/commands/mite';
import { trigger, useOn } from '@/composeables/emit';
import { computed, onMounted, ref, reactive } from 'vue';
import { PlayIcon, StopIcon } from '@heroicons/vue/solid';

const timer = ref(null);

// stopWatch counting up time
const stopWatch = reactive({
  hours: 0,
  minutes: 0,
  interval: null,
});

const isRunning = computed(() => !!timer.value && Object.keys(timer.value).length > 0);

// get minutes
const minutes = computed(() => {
  if (!isRunning.value) {
    return '—';
  }

  return formatNum(stopWatch.minutes);
});

// get hours
const hours = computed(() => {
  if (!isRunning.value) {
    return '—';
  }

  return formatNum(stopWatch.hours);
});

/**
 * Start a timer by creating a new timer instance
 */
function start() {
  Mite.startTimer()
    .then(({tracker}) => {
      trigger('notify', 'Timer started');
      timer.value = tracker.tracking_time_entry;
    })
    .catch(err => console.log(err));

  startInterval();
}

/**
 * Stop current timer
 */
function stop() {
  Mite.stopTimer(timer.value.id)
    .then(() => {
      stopInterval();

      timer.value = {};

    })
    .catch(err => console.log(err));
}

/**
 * Start the interval
 */
function startInterval(hours = 0, minutes = 0) {
  stopWatch.minutes = minutes;
  stopWatch.hours = hours;

  stopWatch.interval = window.setInterval(() => {
    stopWatch.minutes += 1;
  }, 60 * 1000);
}

/**
 * Stop current timer
 */
function stopInterval() {
  stopWatch.minutes = 0;
  stopWatch.hours = 0;
  window.clearInterval(stopWatch.interval);
}

/**
 * Prepend a 0 before number if is smaller than 10
 * 
 * @param {number} n 
 */
function formatNum(n) {
  if (n < 10) {
    return `0${n}`;
  }

  return n;
}

// timer started elsewhere
useOn('started-timer', (tracker) => {
  const { tracking_time_entry } = tracker;

  timer.value = tracking_time_entry ?? {};

  startInterval();
});

onMounted(() => {
  // get current timer
  Mite.getTimer()
    .then(({tracker}) => {
      const { tracking_time_entry } = tracker;

      timer.value = tracking_time_entry ?? {};

      // if tracking time is undefined return
      if (!tracking_time_entry) {
        return;
      }

      // init our stop watch
      startInterval(
        Math.floor(tracking_time_entry.minutes / 60),
        tracking_time_entry.minutes % 60
      );
    })
    .catch(err => console.log(err));
});
</script>

<style scoped>
.timer-wrapper {
  margin-bottom: 2.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  font-size: 1.25rem;
}

.timer {
  font-weight: bold;
}

.timer span {
  display: inline-block;
  margin: 0 0.25rem;
}

.start-stop-btn {
  display: flex;
  justify-content: space-evenly;
  min-width: 5rem;
}

.start-stop-btn .btn {
  margin: 0;
}
</style>
