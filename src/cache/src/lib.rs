pub mod filesystem;
pub mod errors;
pub mod client;

// File system tests
#[cfg(test)]
mod tests {
    use crate::filesystem::FilesystemCache;
    use crate::client::Client;

    #[test]
    fn get_setter() {
        let cache = FilesystemCache::new().unwrap();
        cache.set("test".to_string(), "test".to_string()).unwrap();
        assert_eq!(cache.get("test").unwrap(), "test");

        cache.set("test".to_string(), "test2".to_string()).unwrap();
        assert_eq!(cache.get("test").unwrap(), "test2");
    }

    #[test]
    fn get_not_found() {
        let cache = FilesystemCache::new().unwrap();
        assert_eq!(cache.get("test").is_err(), true);
    }

    #[test]
    fn get_not_found_after_set() {
        let cache = FilesystemCache::new().unwrap();
        cache.set("test".to_string(), "test".to_string()).unwrap();
        assert_eq!(cache.get("test").unwrap(), "test");
        assert_eq!(cache.get("test2").is_err(), true);
    }

    #[test]
    fn overwrite() {
        let cache = FilesystemCache::new().unwrap();
        cache.set("test".to_string(), "test".to_string()).unwrap();
        assert_eq!(cache.get("test").unwrap(), "test");

        cache.set("test".to_string(), "test2".to_string()).unwrap();
        assert_eq!(cache.get("test").unwrap(), "test2");
    }

    #[test]
    fn concurrency() {
        use std::thread;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let cache = Arc::new(FilesystemCache::new().unwrap());
        let counter = Arc::new(AtomicUsize::new(0));
        let mut threads = vec![];

        for _ in 0..10 {
            let cache = Arc::clone(&cache);
            let counter = Arc::clone(&counter);

            threads.push(thread::spawn(move || {
                for _ in 0..100 {
                    let key = counter.fetch_add(1, Ordering::SeqCst).to_string();
                    cache.set(key.clone(), key.clone()).unwrap();
                    assert_eq!(cache.get(&key).unwrap(), key);
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
