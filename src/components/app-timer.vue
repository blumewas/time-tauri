<template>
  <div class="timer-wrapper">
    <div class="timer">
      <span v-text="display"></span>
    </div>
    <!-- TODO pause -->
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
import { computed, ref, reactive } from 'vue';
import { PlayIcon, StopIcon } from '@heroicons/vue/solid';
import { minutesAsString } from '@/helper/time-helper';

const timer = ref(null);

// stopWatch counting up time
const stopWatch = reactive({
  display: '— : —',
  interval: null,
});

const isRunning = computed(() => !!timer.value && Object.keys(timer.value).length > 0);

// get display for time
const display = computed(() => {
  if (!isRunning.value) {
    return '— : —';
  }

  return stopWatch.display;
});

/**
 * Start a timer by creating a new timer instance
 */
function start() {
  Mite.startTimer()
    .then(({tracker}) => {
      trigger('notify', 'Timer started');
      timer.value = tracker.tracking_time_entry;

      refresh();
    })
    .catch(err => console.log(err));
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
 * Refresh our timer
 */
function refresh() {
  Mite.getTimer()
    .then((result) => {

      if (!result) {
        return;
      }


      const { tracking_time_entry } = result.tracker;

      timer.value = tracking_time_entry ?? {};

      // if tracking time is undefined return
      if (!tracking_time_entry) {
        return;
      }

      stopWatch.display = minutesAsString(tracking_time_entry.minutes);
    })
    .catch(err => console.log(err));
}

/**
 * Stop current timer
 */
function stopInterval() {
  stopWatch.display = '00:00';
  window.clearInterval(stopWatch.interval);
}


// timer started elsewhere
useOn('started-timer', (tracker) => {
  const { tracking_time_entry } = tracker;

  timer.value = tracking_time_entry ?? {};

  stopWatch.display = minutesAsString(timer.value.minutes);

  startInterval();
});

useOn('loaded-settings', () => {
  refresh();

  startInterval();
});

/**
 * Start the interval
 */
function startInterval() {
  // get timer every 60 seconds and re new the display
  stopWatch.interval = window.setInterval(refresh, 60 * 1000);
}
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
