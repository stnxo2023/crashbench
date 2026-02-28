//

class UserService {
  constructor(db) {
    this.db = db;
    this.editableFields = ['display_name', 'bio', 'email', 'avatar_url', 'preferences'];
  }

  async getById(id) {
    return this.db.users.findOne({ id });
  }

  async create(data) {
    const user = {
      id: this.db.generateId(),
      role: 'user',
      active: true,
      createdAt: new Date().toISOString(),
      ...data,
    };
    await this.db.users.insert(user);
    return user;
  }

  async update(id, updates) {
    const user = await this.getById(id);
    if (!user) throw new Error('User not found');

    const filtered = {};
    for (const key of this.editableFields) {
      if (updates[key] !== undefined) filtered[key] = updates[key];
    }

    const updated = { ...user, ...filtered, updatedAt: new Date().toISOString() };
    await this.db.users.update({ id }, updated);
    return updated;
  }

  async deactivate(id) {
    return this.db.users.update({ id }, { active: false });
  }

  async list(page = 1, limit = 20) {
    const offset = (page - 1) * limit;
    return this.db.users.find({}, { offset, limit });
  }
}

module.exports = UserService;
