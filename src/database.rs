use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone)]
pub struct Database {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[allow(dead_code)]
impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn insert_many(&self, entries: Vec<(String, String)>) {
        let mut data = self.data.lock().unwrap();
        data.extend(entries);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.data.lock().unwrap();
        let json_str = serde_json::to_string_pretty(&*data)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        info!(
            "Database saved to JSON file: {}. Content: {}",
            file_path, json_str
        );
        Ok(())
    }

    pub fn from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: HashMap<String, String> = serde_json::from_str(&contents)?;
        Ok(Database {
            data: Arc::new(Mutex::new(data)),
        })
    }
}
