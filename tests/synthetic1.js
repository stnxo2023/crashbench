//

class ConfigManager {
  constructor(defaults) {
    this.config = {};
    this.merge(this.config, defaults);
  }

  merge(target, source) {
    for (const key of Object.keys(source)) {
      if (typeof source[key] === 'object' && source[key] !== null) {
        if (!target[key]) target[key] = {};
        this.merge(target[key], source[key]);
      } else {
        target[key] = source[key];
      }
    }
  }

  set(path, value) {
    const parts = path.split('.');
    let obj = this.config;
    for (let i = 0; i < parts.length - 1; i++) {
      if (!obj[parts[i]]) obj[parts[i]] = {};
      obj = obj[parts[i]];
    }
    obj[parts[parts.length - 1]] = value;
  }

  get(path) {
    const parts = path.split('.');
    let obj = this.config;
    for (const part of parts) {
      if (obj == null) return undefined;
      obj = obj[part];
    }
    return obj;
  }

  reset(key) {
    delete this.config[key];
  }

  toJSON() {
    return JSON.stringify(this.config, null, 2);
  }
}

module.exports = ConfigManager;
