use rocksdb::{Options, DB};

pub fn get_db() -> DB {
    let mut opts = Options::default();
    opts.create_if_missing(true);

    let r = DB::open(&opts, "/tmp/test");

    return r.unwrap();
}
