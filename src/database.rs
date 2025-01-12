use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

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

    pub fn insert_many(&self, entries: Vec<(String, String)>) {
        let mut data = self.data.lock().unwrap();
        data.extend(entries);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        const DATABASE_FILE: &str = "database.json";

        // ... in your to_json and from_json methods ...

        // Acquire the lock on the data
        let data = self.data.lock().unwrap();

        // Serialize the data to a JSON string
        let json_str = serde_json::to_string_pretty(&*data)?;

        // Create a file and write the JSON string to it
        let mut file = File::create(DATABASE_FILE)?;
        file.write_all(json_str.as_bytes())?;

        info!("Database saved to JSON file: {}", file_path);

        Ok(())
    }

    pub fn from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        const DATABASE_FILE: &str = "database.json";

        // ... in your to_json and from_json methods ...



        let mut file = File::open(DATABASE_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: HashMap<String, String> = serde_json::from_str(&contents)?;
        Ok(Database {
            data: Arc::new(Mutex::new(data)),
        })
    }
}