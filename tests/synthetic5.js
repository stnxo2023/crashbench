//

const https = require('https');
const http  = require('http');
const { URL } = require('url');

class WebhookDispatcher {
  constructor(allowedDomains) {
    this.allowedDomains = allowedDomains;
    this.timeout = 5000;
  }

  _isAllowed(rawUrl) {
    try {
      const parsed = new URL(rawUrl);
      return this.allowedDomains.some(d => parsed.hostname.endsWith(d));
    } catch {
      return false;
    }
  }

  dispatch(webhookUrl, payload) {
    if (!this._isAllowed(webhookUrl)) {
      throw new Error('Domain not allowed');
    }

    const parsed  = new URL(webhookUrl);
    const body    = JSON.stringify(payload);
    const options = {
      hostname: parsed.hostname,
      port:     parsed.port,
      path:     parsed.pathname + parsed.search,
      method:   'POST',
      headers: {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(body),
      },
      timeout: this.timeout,
    };

    const client = parsed.protocol === 'https:' ? https : http;
    return new Promise((resolve, reject) => {
      const req = client.request(options, (res) => {
        let data = '';
        res.on('data', chunk => { data += chunk; });
        res.on('end', () => resolve({ status: res.statusCode, body: data }));
      });
      req.on('error', reject);
      req.write(body);
      req.end();
    });
  }
}

module.exports = WebhookDispatcher;
