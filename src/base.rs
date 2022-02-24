use crate::db::*;
use crate::option::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rocksdb::{Options, DB};
use std::sync::Arc;
use std::time::Duration;

create_exception!(rocksdbpy, RocksDBPyException, PyException);

/// Opens a database with default options.
///
/// # Example
///
/// ```
/// db = RocksDB()
/// rocksdbpy.open_default('/tmp/test')
/// ```
#[pyfunction]
pub fn open_default(path: &str) -> PyResult<RocksDBPy> {
    match DB::open_default(path) {
        Ok(db) => {
            let db = RocksDBPy {
                db: Arc::new(db),
                path: path.as_bytes().to_vec(),
            };

            return Ok(db);
        }
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be open, {}",
            e
        ))),
    }
}

/// Opens the database with the specified options.
///
/// # Example
///
/// ```
/// opts = Option()
/// opts.create_if_missing(True)
///
/// rocksdbpy.open('/tmp/test', opts)
/// ```
#[pyfunction]
pub fn open(path: &str, opts: &OptionPy) -> PyResult<RocksDBPy> {
    match DB::open(&opts.inner, path) {
        Ok(db) => {
            let db = RocksDBPy {
                db: Arc::new(db),
                path: path.as_bytes().to_vec(),
            };

            return Ok(db);
        }
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be open, {}",
            e
        ))),
    }
}

/// Opens the database with TTL compaction filter.
///
/// # Example
///
/// ```
/// opts = Option()
/// opts.create_if_missing(True)
///
/// rocksdbpy.open_with_ttl('/tmp/test', 5, opts)
/// ```
#[pyfunction]
pub fn open_with_ttl(path: &str, ttl: u64, opts: &OptionPy) -> PyResult<RocksDBPy> {
    let duration = Duration::from_secs(ttl);

    match DB::open_with_ttl(&opts.inner, path, duration) {
        Ok(db) => {
            let db = RocksDBPy {
                db: Arc::new(db),
                path: path.as_bytes().to_vec(),
            };

            return Ok(db);
        }
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be open with {} with ttl {} seconds. {}",
            path,
            duration.as_secs(),
            e,
        ))),
    }
}

/// Destroy database and it's files.
///
/// # Example
///
/// ```
/// rocksdbpy.destroy('/tmp/test')
/// ```
#[pyfunction]
pub fn destroy(path: &str, option: Option<OptionPy>) -> PyResult<()> {
    let mut opts: Options = Options::default();

    if !option.is_none() {
        opts = option.unwrap().inner;
    }

    match DB::destroy(&opts, path) {
        Ok(()) => Ok(()),
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be destory, {}",
            e
        ))),
    }
}
