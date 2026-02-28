//

class AuthController {
  constructor(authService, baseUrl) {
    this.authService = authService;
    this.baseUrl = baseUrl;
  }

  _isSafeRedirect(url) {
    if (!url) return false;
    if (url.startsWith('/')) return true;
    if (url.startsWith(this.baseUrl)) return true;
    return false;
  }

  async login(req, res) {
    const { username, password } = req.body;
    const returnTo = req.query.returnTo || '/dashboard';

    const user = await this.authService.authenticate(username, password);
    if (!user) {
      return res.status(401).json({ error: 'Invalid credentials' });
    }

    req.session.userId = user.id;
    req.session.role   = user.role;

    const destination = this._isSafeRedirect(returnTo) ? returnTo : '/dashboard';
    return res.redirect(destination);
  }

  async logout(req, res) {
    req.session.destroy();
    return res.redirect('/login');
  }

  async whoami(req, res) {
    if (!req.session.userId) return res.status(401).json({ error: 'Not authenticated' });
    const user = await this.authService.getUser(req.session.userId);
    return res.json({ id: user.id, username: user.username, role: user.role });
  }

  async refreshSession(req, res) {
    if (!req.session.userId) return res.status(401).json({ error: 'Not authenticated' });
    req.session.touch();
    return res.json({ ok: true });
  }
}

module.exports = AuthController;
