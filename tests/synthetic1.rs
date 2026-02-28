//

use std::collections::HashMap;

pub struct DataStore {
    records: HashMap<u32, Vec<u8>>,
    max_size: u32,
    current_size: u32,
}

impl DataStore {
    pub fn new(max_size: u32) -> Self {
        DataStore {
            records: HashMap::new(),
            max_size,
            current_size: 0,
        }
    }

    pub fn insert(&mut self, id: u32, data: Vec<u8>) -> Result<(), String> {
        let data_len = data.len() as u32;

        if self.current_size + data_len > self.max_size {
            return Err("Store capacity exceeded".to_string());
        }

        self.current_size += data_len;
        self.records.insert(id, data);
        Ok(())
    }

    pub fn remove(&mut self, id: u32) -> Option<Vec<u8>> {
        if let Some(data) = self.records.remove(&id) {
            self.current_size -= data.len() as u32;
            return Some(data);
        }
        None
    }

    pub fn get(&self, id: u32) -> Option<&Vec<u8>> {
        self.records.get(&id)
    }

    pub fn utilization(&self) -> f64 {
        self.current_size as f64 / self.max_size as f64
    }
}
