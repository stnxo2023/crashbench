//

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AuditLogger {
    entries: Vec<String>,
    context: HashMap<String, String>,
}

impl AuditLogger {
    pub fn new() -> Self {
        AuditLogger {
            entries: Vec::new(),
            context: HashMap::new(),
        }
    }

    pub fn set_context(&mut self, key: &str, value: &str) {
        self.context.insert(key.to_string(), value.to_string());
    }

    pub fn log_event(&mut self, event_type: &str, user_input: &str) {
        let ctx = self
            .context
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        let entry = format!("[{}] {} | ctx: {}", event_type, user_input, ctx);
        println!("{}", entry);
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &[String] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl fmt::Display for AuditLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}
