/**
 * Convert passed int to hh::mm string
 * 
 * @param {int} minutes 
 * @returns 
 */
export function minutesAsString(minutes) {

  const h = Math.floor(minutes / 60);
  const m = minutes % 60

  return `${formatNum(h)}:${formatNum(m)}`;
}

/**
 * Prepend a 0 before passed Number
 * 
 * @param {int} n 
 * @returns String
 */
function formatNum(n) {
  if (n < 10) {
    return `0${n}`;
  }

  return n;
}
