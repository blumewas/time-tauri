import { onUnmounted, onBeforeMount } from 'vue';

export function useOn(name, callback) {
  
  // add event listener on when create component
  onBeforeMount(() => {
    window.addEventListener(name, (event) => {
      callback(event.detail);
    });
  });

  // remove event listener on when umounting component
  onUnmounted(() => {
    window.removeEventListener(name, callback);
  });

}

export function trigger(name, detail) {
  window.dispatchEvent(new CustomEvent(name, { detail }));
}
