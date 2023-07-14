use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use dirs;
use crate::client::Client;
use crate::errors::CacheError;

#[derive(Debug)]
pub struct FilesystemCache {
    path: PathBuf,
    lock: Mutex<()>
}

// File system cache is a simple key-value store that uses a txt file to store data.
impl FilesystemCache {
    pub fn new() -> Result<Self, CacheError> {
        let mut path = dirs::cache_dir().expect("Unable to get cache directory");
        path.push("howfast_cache.txt");

        if !path.exists() {
            File::create(&path).unwrap();
        }

        Ok(Self {
            path,
            lock: Mutex::new(()),
        })
    }

    fn get_store(&self) -> Result<HashMap<String, String>, CacheError> {
        let mut store = HashMap::new();
        let guard = self.lock.lock().unwrap();

        let mut file = File::open(&self.path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        store = serde_json::from_str(&data).map_err(CacheError::ParseError)?;

        drop(guard);

        Ok(store)
    }
}

// Implement the client trait for the filesystem cache
impl Client for FilesystemCache {
    fn get(&self, key: &str) -> Result<String, CacheError> {
        let guard = self.lock.lock().unwrap();
        let store = self.get_store().unwrap();

        let result = store.get(key).cloned().ok_or(CacheError::NotFound);

        drop(guard);

        result
    }

    fn set(&self, key: String, value: String) -> Result<(), CacheError> {
        let mut guard = self.lock.lock().unwrap();
        let mut store = self.get_store().unwrap();

        store.insert(key, value);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        let  data =  serde_json::to_string(&store).map_err(CacheError::SerializeError)?;

        file.write_all(data.as_bytes())?;

        drop(guard);

        Ok(())
    }
}