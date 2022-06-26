import { invoke } from "@tauri-apps/api";

export class Mite {

  /**
   * Get mite services via tauri
   * 
   * @returns {Object}
   */
  static async getServices() {

    // invoke handler
    const data = await this.handle('get_services');

    // return array of data objects
    return data.map(({service}) => {
      return {
        id: service.id,
        name: service.name,
      };
    });
  }

  /**
   * Get mite projects via tauri
   * 
   * @returns {Object}
   */
  static async getProjects() {
    // invoke handler
    const data = await this.handle('get_projects');

    // flatten returned value
    return data.map(({project}) => project);
  }

  /**
   * Get current timer from mite
   * 
   * @returns {Object}
   */
  static async getTimer() {
    const timer = await this.handle('get_timer');

    return timer;
  }

  /**
   * Start a timer
   * 
   * @param {number} projectId 
   * @param {number} serviceId 
   */
  static async startTimer(projectId = null, serviceId = null) {
    const created = await this.handle('create_time', { projectId, serviceId });

    const entryId = created.time_entry.id;

    const timer = await this.handle('start_timer', { entryId });

    return timer;
  }

  /**
   * Stop current timer
   * 
   * @param {number} timerId 
   * @returns 
   */
  static async stopTimer(timerId) {
    const result = await this.handle('stop_timer', { timerId });
    
    return result;
  }

  /**
   * Create time entry
   * 
   * @param {number} projectId 
   * @param {number} serviceId 
   * @param {number} minutes 
   * @param {string} note 
   */
  static async createTime(projectId = null, serviceId = null, minutes = 0, note = "") {
    await this.handle('create_time', { projectId, serviceId, minutes, note });
  }

  /**
   * Invoke passed command and return either
   * parsed json data or parsed error message 
   * 
   * @param {string} cmd 
   * @returns {Object}
   * @throws {Error}
   */
  static async handle(cmd, args = null) {
    try {
      const message = await invoke(cmd, args);
      const data = JSON.parse(message);

      return data;
    } catch (error) {
      // check if we have content
      if (!error) {
        return;
      }

      if (this.isJson(error)) {
        // parse error we get form tauri
        const err = JSON.parse(error);
        throw new Error(err.error);
      }
      
      throw error;
    }
  }

  static isJson(str) {
    try {
      JSON.parse(str);
    } catch (e) {
      return false;
    }
    return true;
  }

}
