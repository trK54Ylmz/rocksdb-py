mod db;
mod function;

use crate::db::*;
use crate::function::*;
use rocksdb::DB;

fn main() {
    let size = 10_000;
    let p = function::PREFIX;
    let path = "/tmp/test";

    // Get RocksDB connection
    let db = get_db(path);

    let get_key_f = |db: &DB, size: i32| get(db, format!("{}_{}", p, size).as_str());
    let put_key_f = |db: &DB, size: i32| put(db, format!("{}_{}", p, size).as_str(), "value");

    let get_key_diff = timeit(get_key_f, size, &db);
    let put_key_diff = timeit(put_key_f, size, &db);

    println!("Put key = {}μs", put_key_diff);
    println!("Get key = {}μs", get_key_diff);

    // Destroy database
    destroy(db, path);
}
