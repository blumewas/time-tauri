import {appDir} from '@tauri-apps/api/path';
import {readTextFile} from '@tauri-apps/api/fs';
import dayjs from 'dayjs';

export class ActivityLog {

  /**
   * Get log file content for specified date
   * 
   * @param {*} date 
   * @returns 
   */
  static async day(date) {
    const d = dayjs(date).format('YYYY-MM-DD_ddd');

    const path = await appDir();
      
    const logContent = await readTextFile(`${path}activity-logs/${d}.txt`);

    const entries = [];
    const lines = logContent.split("\n");
    for (const line of lines) {
      const entry = this.toObj(line);
      if (!entry) {
        continue;
      }

      entries.push(entry);
    }

    return entries;
  }

  /**
   * Group entry array by window  name
   * 
   * @param {*} entries 
   */
  static groupByApp(entries) {
    return entries.reduce((apps, entry) => {
      const { app, window, time } = entry;
      const t = Number.parseInt(time.replace('s', ''), 10);

      if (!apps[app]) {
        apps[app] = {
          windows: [window],
          total: t,
        };

        return apps;
      }

      if (!apps[app].windows.includes(window)) {
        apps[app].windows.push(window);
      }

      apps[app].total += t;

      return apps;
    }, {});
  }

  /**
   * Convert log line to object
   * 
   * @param {*} line 
   * @returns 
   */
  static toObj(line) {
    const split = line.split(', ');
    if (split.length !== 3) {
      return null;
    }

    return {
      app: split[0],
      window: split[1],
      time: split[2],
    };
  }

}
