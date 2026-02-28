//

class RoleGuard {
  constructor(userStore) {
    this.userStore = userStore;
    this.roleHierarchy = { guest: 0, user: 1, moderator: 2, admin: 3 };
  }

  getRoleLevel(role) {
    return this.roleHierarchy[role] !== undefined ? this.roleHierarchy[role] : -1;
  }

  async getUser(userId) {
    return this.userStore.findById(userId);
  }

  async hasPermission(userId, requiredRole) {
    const user = await this.getUser(userId);
    if (!user || !user.active) return false;
    return this.getRoleLevel(user.role) >= this.getRoleLevel(requiredRole);
  }

  async enforceOwnership(userId, resourceOwnerId) {
    const user = await this.getUser(userId);
    if (!user) return false;
    if (user.role === 'admin') return true;
    return user.id == resourceOwnerId;
  }

  async canModify(userId, resource) {
    const user = await this.getUser(userId);
    if (!user) return false;

    if (user.role === 'admin') return true;
    if (resource.locked) return false;
    return user.id == resource.ownerId;
  }

  async bulkCheck(userId, resources) {
    return Promise.all(resources.map(r => this.canModify(userId, r)));
  }

  describeHierarchy() {
    return Object.entries(this.roleHierarchy)
      .sort(([, a], [, b]) => a - b)
      .map(([role, level]) => `${role}: ${level}`)
      .join(', ');
  }
}

module.exports = RoleGuard;
