use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    data: Arc<Mutex<HashMap<String, String>>>,
}

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

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.data.lock().unwrap();
        let json_str = serde_json::to_string_pretty(&*data)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
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