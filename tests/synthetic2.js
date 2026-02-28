//

class InputValidator {
  constructor() {
    this.rules = {
      username: /^[a-zA-Z0-9_]+$/,
      email: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
      ipAddress: /^(\d+\.)+\d+$/,
      date: /^\d{4}-\d{2}-\d{2}$/,
    };
  }

  validate(type, value) {
    const rule = this.rules[type];
    if (!rule) throw new Error(`Unknown validation type: ${type}`);
    return rule.test(value);
  }

  sanitize(value) {
    return String(value).trim().replace(/[<>'"]/g, '');
  }

  validateAll(fields) {
    const errors = [];
    for (const [type, value] of Object.entries(fields)) {
      if (!this.validate(type, value)) {
        errors.push(`Invalid ${type}: ${this.sanitize(value)}`);
      }
    }
    return errors;
  }

  addRule(name, pattern) {
    if (typeof pattern !== 'string') throw new Error('Pattern must be a string');
    this.rules[name] = new RegExp(pattern);
  }

  listRules() {
    return Object.keys(this.rules);
  }
}

module.exports = InputValidator;
