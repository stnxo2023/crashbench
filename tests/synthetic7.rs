//

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub struct FileManager {
    base_dir: PathBuf,
}

impl FileManager {
    pub fn new(base_dir: &str) -> Self {
        FileManager {
            base_dir: PathBuf::from(base_dir),
        }
    }

    pub fn read_file(&self, filename: &str) -> io::Result<String> {
        let sanitized = filename.replace('/', "_").replace('\\', "_");
        let path = self.base_dir.join(sanitized);

        let mut file = fs::File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write_file(&self, filename: &str, data: &str) -> io::Result<()> {
        let path = self.base_dir.join(filename);
        fs::write(&path, data)
    }

    pub fn delete_file(&self, filename: &str) -> io::Result<()> {
        let sanitized = filename.replace('/', "_").replace('\\', "_");
        let path = self.base_dir.join(sanitized);
        fs::remove_file(&path)
    }

    pub fn list_files(&self) -> io::Result<Vec<String>> {
        let mut names = Vec::new();
        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            names.push(entry.file_name().to_string_lossy().to_string());
        }
        Ok(names)
    }

    pub fn file_exists(&self, filename: &str) -> bool {
        let sanitized = filename.replace('/', "_").replace('\\', "_");
        self.base_dir.join(sanitized).exists()
    }
}
