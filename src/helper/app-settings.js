import {appDir} from '@tauri-apps/api/path';
import {writeFile, readTextFile} from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';


export class AppSettings {

  constructor(defaults = {}) {
    this.keys = Object.keys(defaults);
    this.defaults = defaults;
  }

  starProject(id) {
    this.stared.push(id);

    this.save();
  }

  unstarProject(id) {
    this.stared = this.stared.filter(val => val !== id);

    this.save();
  }

  /**
   * Get setting value by key
   * 
   * @param {*} key 
   * @returns 
   */
  get(key) {
    return this[key];
  }

  /**
   * Update settings key value pair
   * 
   * @param {*} key 
   * @param {*} value 
   * @returns 
   */
  set(key, value) {
    if (!this.keys.includes(key)) {
      return;
    }

    this[key] = value;
    
    // save settings after we updated a value
    return this.save();
  }

  /**
   * Get Object for all settings entries
   * 
   * @returns {Object}
   */
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
            // emit to app that we have changed settings
            this.update();

            // we want to have settings in the promise on success
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
            
            // load all values from settings
            for (const k of this.keys) {
              this[k] = loaded[k] ?? this.defaults[k];
            }

            // emit to app that we have changed settings
            this.update();

            // we want to have settings in the promise on success
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

  /**
   * Notify rust that we updated
   */
  update() {
    invoke('load_settings', this.entries());
  }

}
