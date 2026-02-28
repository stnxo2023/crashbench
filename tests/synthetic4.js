//

class UserRepository {
  constructor(db) {
    this.db = db;
    this.table = 'users';
  }

  async findById(id) {
    return this.db.query('SELECT * FROM users WHERE id = ?', [id]);
  }

  async findByEmail(email) {
    return this.db.query('SELECT * FROM users WHERE email = ?', [email]);
  }

  async updateField(id, field, value) {
    const allowed = ['display_name', 'bio', 'avatar_url'];
    if (!allowed.includes(field)) throw new Error('Invalid field');
    return this.db.query(`UPDATE users SET ${field} = ? WHERE id = ?`, [value, id]);
  }

  async search(filters) {
    const conditions = [];
    const params = [];

    for (const [key, val] of Object.entries(filters)) {
      if (val !== undefined && val !== null) {
        conditions.push(`${key} = ?`);
        params.push(val);
      }
    }

    const where = conditions.length ? `WHERE ${conditions.join(' AND ')}` : '';
    const sql = `SELECT id, username, email FROM users ${where}`;
    return this.db.query(sql, params);
  }

  async delete(id) {
    return this.db.query('DELETE FROM users WHERE id = ?', [id]);
  }

  async count() {
    const rows = await this.db.query('SELECT COUNT(*) as total FROM users', []);
    return rows[0].total;
  }
}

module.exports = UserRepository;
