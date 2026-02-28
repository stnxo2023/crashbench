//

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct SecureLogger {
    log_dir: PathBuf,
    max_log_size: u64,
}

impl SecureLogger {
    pub fn new(log_dir: &str, max_log_size: u64) -> Self {
        SecureLogger {
            log_dir: PathBuf::from(log_dir),
            max_log_size,
        }
    }

    pub fn write_log(&self, filename: &str, content: &str) -> io::Result<()> {
        let log_path = self.log_dir.join(filename);

        if log_path.exists() {
            let metadata = fs::metadata(&log_path)?;
            if metadata.len() >= self.max_log_size {
                return Err(io::Error::new(io::ErrorKind::Other, "Log file too large"));
            }
        }

        if !self.is_within_log_dir(&log_path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Invalid path"));
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn is_within_log_dir(&self, path: &Path) -> bool {
        path.starts_with(&self.log_dir)
    }

    pub fn list_logs(&self) -> io::Result<Vec<String>> {
        let mut logs = Vec::new();
        for entry in fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            logs.push(entry.file_name().to_string_lossy().to_string());
        }
        Ok(logs)
    }
}
