use rocksdb::DB;
use std::time::Instant;

pub const PREFIX: &str = "test_";

/// Measure running time of the given function
pub fn timeit<'a, F>(f: F, size: i32, db: &'a DB) -> u128
where
    F: Fn(&'a DB, i32),
{
    let start = Instant::now();

    for _ in 0..size - 1 {
        f(db, size);
    }

    return start.elapsed().as_micros();
}

/// Get value by given key
pub fn get<'a>(db: &'a DB, key: &str) -> Result<Option<String>, String> {
    // Get value by given key
    match db.get(key) {
        Ok(None) => Ok(None),
        Ok(Some(value)) => Ok(Some(String::from_utf8(value).unwrap())),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Set entry for given key and value
pub fn put<'a>(db: &'a DB, key: &str, value: &str) {
    // Set key and value
    db.put(key, value).unwrap();
}
