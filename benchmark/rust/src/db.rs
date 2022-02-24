use rocksdb::{Options, DB};

/// Open the database by given path
pub fn get_db(path: &str) -> DB {
    let mut opts = Options::default();
    opts.create_if_missing(true);

    let r = DB::open(&opts, path);

    return r.unwrap();
}

/// Close active database and destroy
pub fn destroy(db: DB, path: &str) {
    drop(db);

    let opts = Options::default();

    DB::destroy(&opts, path).unwrap();
}
