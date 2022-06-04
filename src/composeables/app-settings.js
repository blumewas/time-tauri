import {appDir} from '@tauri-apps/api/path';
import {writeFile, readTextFile} from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';


export class AppSettings {

  constructor(defaults = {}) {
    this.keys = Object.keys(defaults);
    this.defaults = defaults;
  }

  get(key) {
    return this[key];
  }

  set(key, value) {
    if (!this.keys.includes(key)) {
      return;
    }

    this[key] = value;
    console.log('saving settings');

    return this.save();
  }

  entries() {
    const settings = {};
    for (const k of this.keys) {
      settings[k] = this[k];
    }

    return settings;
  }

  /**
   * Save settings to file
   */
  save() {
    return new Promise((resolve, reject) => {

      appDir().then(path => {
        const settingsJson = JSON.stringify(this.entries());

        writeFile({contents: settingsJson, path: `${path}settings.txt`})
          .then(() => {
            this.update();

            resolve(this);
          })
          .catch(() => {
            reject('error while writing file')
          });
      });
    });
  }

   /**
   * Save settings from file
   */
  load() {
    return new Promise((resolve, reject) => {
      appDir().then(path => {
        readTextFile(`${path}settings.txt`)
          .then((contents) => {
            const loaded = JSON.parse(contents);
      
            for (const k of this.keys) {
              this[k] = loaded[k] ?? this.defaults[k];
            }
            
            // emit to app that we have loaded our app
            this.update();

            resolve(this);
          })
          .catch((err) => {
            // try to write default settings here
            const settingsJson = JSON.stringify(this.defaults);
      
            writeFile({contents: settingsJson, path: `${path}settings.txt`});

            reject(err);
          });
      });
    });
  }

  update() {
    invoke('load_settings', this.entries());
  }

}
