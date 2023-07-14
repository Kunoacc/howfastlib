use crate::errors::CacheError;

pub trait Client {
    fn get(&self, key: &str) -> Result<String, CacheError>;
    fn set(&self, key: String, value: String) -> Result<(), CacheError>;
}