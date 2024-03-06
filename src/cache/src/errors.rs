use std::io;

#[derive(Debug)]
pub enum CacheError {
    NotFound,
    IoError(std::io::Error),
    SerializeError(serde_json::Error),
    ParseError(serde_json::Error),
    CacheDirError,
}

impl From<io::Error> for CacheError {
    fn from(error: io::Error) -> Self {
        CacheError::IoError(error)
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(error: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match error.classify() {
            Category::Io => CacheError::SerializeError(error),
            Category::Syntax | Category::Data => CacheError::ParseError(error),
            _ => CacheError::SerializeError(error),
        }
    }
}