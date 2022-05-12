use rocksdb::{WriteBatch, DB};
use time;

pub const PREFIX: &str = "test";

/// Measure running time of the given function
pub fn timeit<'a, F>(f: F, size: i32, db: &'a DB) -> u64
where
    F: Fn(&'a DB, i32),
{
    let start = time::precise_time_ns();

    for i in 0..size {
        f(db, i);
    }

    let end = time::precise_time_ns();

    return (end - start) / 1000;
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

/// Set multiple entries for given group of keys and values
pub fn put_multi<'a>(db: &'a DB, keys: &[&str], values: &[&str]) {
    let mut b = WriteBatch::default();

    for i in 0..keys.len() {
        b.put(keys[i], values[i])
    }

    db.write(b).unwrap()
}
