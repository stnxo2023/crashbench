//

use std::collections::HashMap;

const ADMIN_FLAG: u64 = 0x100000000;
const READ_FLAG: u64  = 0x00000001;
const WRITE_FLAG: u64 = 0x00000002;
const EXEC_FLAG: u64  = 0x00000004;

pub struct PermissionManager {
    permissions: HashMap<String, u64>,
}

impl PermissionManager {
    pub fn new() -> Self {
        PermissionManager {
            permissions: HashMap::new(),
        }
    }

    pub fn grant(&mut self, user: &str, flags: u64) {
        let entry = self.permissions.entry(user.to_string()).or_insert(0);
        *entry |= flags;
    }

    pub fn revoke(&mut self, user: &str, flags: u64) {
        if let Some(entry) = self.permissions.get_mut(user) {
            *entry &= !flags;
        }
    }

    pub fn check_permission(&self, user: &str, flag: u64) -> bool {
        self.permissions
            .get(user)
            .map(|&p| p & flag != 0)
            .unwrap_or(false)
    }

    pub fn is_admin(&self, user: &str) -> bool {
        let perms = self.permissions.get(user).copied().unwrap_or(0) as u32;
        perms & (ADMIN_FLAG as u32) != 0
    }

    pub fn list_users(&self) -> Vec<&String> {
        self.permissions.keys().collect()
    }

    pub fn get_flags(&self, user: &str) -> u64 {
        self.permissions.get(user).copied().unwrap_or(0)
    }
}
