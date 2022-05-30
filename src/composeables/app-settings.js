import {appDir} from '@tauri-apps/api/path';
import {writeFile, readTextFile} from '@tauri-apps/api/fs';


// TODO add set/get method that directly writes settings to config
export class AppSettings {

  constructor(defaults = {}) {
    this.keys = Object.keys(defaults);
    this.defaults = defaults;
  }

  save() {
    appDir().then(path => {

      const settings = {};
      for (const k of this.keys) {
        settings[k] = this[k];
      }

      const settingsJson = JSON.stringify(settings);

      writeFile({contents: settingsJson, path: `${path}settings.txt`});
    });
  }

  load() {
    return new Promise((resolve, reject) => {
      appDir().then(path => {
        readTextFile(`${path}settings.txt`)
          .then((contents) => {
            const loaded = JSON.parse(contents);
      
            for (const k of this.keys) {
              this[k] = loaded[k] ?? this.defaults[k];
            }
            
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

}
