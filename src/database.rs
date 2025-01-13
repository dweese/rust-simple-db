use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

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

        // Use OpenOptions to create or truncate the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let database = Database::new();
        database.insert("key1".to_string(), "value1".to_string());
        assert_eq!(database.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_insert_many() {
        let database = Database::new();
        let entries = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        database.insert_many(entries);
        assert_eq!(database.get("key1"), Some("value1".to_string()));
        assert_eq!(database.get("key2"), Some("value2".to_string()));
    }

    #[test]
    fn test_to_json_and_from_json() {
        let database = Database::new();
        database.insert("key1".to_string(), "value1".to_string());
        database.to_json("test_db.json").unwrap();

        let loaded_db = Database::from_json("test_db.json").unwrap();
        assert_eq!(loaded_db.get("key1"), Some("value1".to_string()));

        // Clean up the test file
        std::fs::remove_file("test_db.json").unwrap();
    }
}
