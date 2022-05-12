mod db;
mod function;

use crate::db::*;
use crate::function::*;
use rocksdb::DB;

fn main() {
    let size = 100000;
    let p = function::PREFIX;
    let path = "/tmp/test";

    // Get RocksDB connection
    let db = get_db(path);

    let get_key_f = |db: &DB, size: i32| {
        get(db, format!("{}_{}", p, size).as_str()).unwrap();
    };

    let put_key_f = |db: &DB, size: i32| {
        put(db, format!("{}_{}", p, size).as_str(), "value");
    };

    let put_m_key_f = |db: &DB, size: i32| {
        let mut kv = vec!["".to_owned(); 10];
        let mut vv = vec!["".to_owned(); 10];

        for i in 0..10 {
            kv[i] = format!("{}_{}_{}", p, size, i).to_owned();
            vv[i] = "value".to_owned();
        }

        let keys: Vec<_> = kv.iter().map(String::as_str).collect();
        let values: Vec<_> = vv.iter().map(String::as_str).collect();

        put_multi(db, &keys, &values)
    };

    let put_key_diff = timeit(put_key_f, size, &db);
    let get_key_diff = timeit(get_key_f, size, &db);
    let put_m_key_diff = timeit(put_m_key_f, size, &db);

    println!("Put key = {}μs", put_key_diff);
    println!("Get key = {}μs", get_key_diff);
    println!("Put multi key = {}μs", put_m_key_diff);

    // Destroy database
    destroy(db, path);
}
