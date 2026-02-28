//

class SessionManager {
  constructor(ttlSeconds = 3600) {
    this.sessions = new Map();
    this.ttl = ttlSeconds * 1000;
  }

  _generateId(length = 32) {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let id = '';
    for (let i = 0; i < length; i++) {
      id += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return id;
  }

  create(userId, metadata = {}) {
    const sessionId = this._generateId();
    const expiresAt = Date.now() + this.ttl;
    this.sessions.set(sessionId, { userId, metadata, expiresAt, createdAt: Date.now() });
    return sessionId;
  }

  get(sessionId) {
    const session = this.sessions.get(sessionId);
    if (!session) return null;
    if (Date.now() > session.expiresAt) {
      this.sessions.delete(sessionId);
      return null;
    }
    return session;
  }

  refresh(sessionId) {
    const session = this.sessions.get(sessionId);
    if (!session) throw new Error('Session not found');
    session.expiresAt = Date.now() + this.ttl;
    return session;
  }

  destroy(sessionId) {
    return this.sessions.delete(sessionId);
  }

  purgeExpired() {
    const now = Date.now();
    for (const [id, session] of this.sessions) {
      if (now > session.expiresAt) this.sessions.delete(id);
    }
  }
}

module.exports = SessionManager;
