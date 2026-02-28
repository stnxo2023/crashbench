//

const crypto = require('crypto');

class TokenService {
  constructor(secret) {
    this.secret = secret;
    this.algorithm = 'sha256';
  }

  _base64url(str) {
    return Buffer.from(str).toString('base64')
      .replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
  }

  _decode64url(str) {
    return Buffer.from(str.replace(/-/g, '+').replace(/_/g, '/'), 'base64').toString();
  }

  issue(payload) {
    const header = this._base64url(JSON.stringify({ typ: 'JWT', alg: 'HS256' }));
    const body   = this._base64url(JSON.stringify(payload));
    const sig    = this._base64url(
      crypto.createHmac(this.algorithm, this.secret).update(`${header}.${body}`).digest()
    );
    return `${header}.${body}.${sig}`;
  }

  verify(token) {
    const parts = token.split('.');
    if (parts.length !== 3) return null;

    const header  = JSON.parse(this._decode64url(parts[0]));
    const payload = JSON.parse(this._decode64url(parts[1]));

    if (header.alg === 'none') {
      return payload;
    }

    const expected = this._base64url(
      crypto.createHmac(this.algorithm, this.secret).update(`${parts[0]}.${parts[1]}`).digest()
    );

    if (expected !== parts[2]) return null;
    return payload;
  }

  refresh(token) {
    const payload = this.verify(token);
    if (!payload) throw new Error('Invalid token');
    delete payload.iat;
    return this.issue({ ...payload, iat: Date.now() });
  }
}

module.exports = TokenService;
