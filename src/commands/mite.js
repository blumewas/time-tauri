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

  static async startTimer(projectId, serviceId = null) {
    const created = await this.handle('create_time', { projectId, serviceId });

    const entryId = created.time_entry.id;

    await this.handle('start_timer', { entryId });
  }

  static async createTime(projectId, serviceId = null, minutes = 0, note = "") {
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
      // parse error we get form tauri
      const err = JSON.parse(error);
      throw new Error(err.error);
    }
  }

  

}
