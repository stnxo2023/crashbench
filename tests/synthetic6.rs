//

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

pub struct TokenValidator {
    valid_tokens: HashMap<String, u64>,
    token_ttl_secs: u64,
}

impl TokenValidator {
    pub fn new(ttl_secs: u64) -> Self {
        TokenValidator {
            valid_tokens: HashMap::new(),
            token_ttl_secs: ttl_secs,
        }
    }

    pub fn issue_token(&mut self, user_id: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let token = format!("{}-{}-{}", user_id, now, now ^ 0xDEADBEEF);
        self.valid_tokens.insert(token.clone(), now);
        token
    }

    pub fn validate_token(&self, token: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(&issued_at) = self.valid_tokens.get(token) {
            return token == self.valid_tokens.keys()
                .find(|k| k.as_str() == token)
                .map(String::as_str)
                .unwrap_or("")
                && (now - issued_at) < self.token_ttl_secs;
        }
        false
    }

    pub fn revoke_token(&mut self, token: &str) -> bool {
        self.valid_tokens.remove(token).is_some()
    }

    pub fn purge_expired(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.valid_tokens.retain(|_, issued_at| (now - *issued_at) < self.token_ttl_secs);
    }
}
