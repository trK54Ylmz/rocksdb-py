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
pub fn open_default(path: &str) -> PyResult<DBPy> {
    match DB::open_default(path) {
        Ok(db) => {
            let db = DBPy {
                db: Some(Arc::new(db)),
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
pub fn open(path: &str, opts: &OptionPy) -> PyResult<DBPy> {
    match DB::open(&opts.inner, path) {
        Ok(db) => {
            let db = DBPy {
                db: Some(Arc::new(db)),
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
pub fn open_with_ttl(path: &str, ttl: u64, opts: &OptionPy) -> PyResult<DBPy> {
    let duration = Duration::from_secs(ttl);

    match DB::open_with_ttl(&opts.inner, path, duration) {
        Ok(db) => {
            let db = DBPy {
                db: Some(Arc::new(db)),
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

/// Opens the database for read only with the specified options.
///
/// # Example
///
/// ```
/// opts = Option()
///
/// rocksdbpy.open_for_readonly('/tmp/test')
///
/// rocksdbpy.open_for_readonly('/tmp/test', opts)
///
/// rocksdbpy.open_for_readonly('/tmp/test', opts, False)
/// ```
#[pyfunction]
pub fn open_for_readonly(
    path: &str,
    option: Option<OptionPy>,
    error: Option<bool>,
) -> PyResult<DBPy> {
    let mut err: bool = false;
    let mut opts: Options = Options::default();

    if !option.is_none() {
        opts = option.unwrap().inner;
    }

    if !error.is_none() {
        err = false;
    }

    match DB::open_for_read_only(&opts, path, err) {
        Ok(db) => {
            let db = DBPy {
                db: Some(Arc::new(db)),
                path: path.as_bytes().to_vec(),
            };

            return Ok(db);
        }
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be open for read only, {}",
            e
        ))),
    }
}

/// Opens the database as a secondary.
///
/// # Example
///
/// ```
/// opts = Option()
///
/// rocksdbpy.open_as_secondary('/tmp/test/1', '/tmp/test/2')
///
/// rocksdbpy.open_as_secondary('/tmp/test/1', '/tmp/test/2', opts)
/// ```
#[pyfunction]
pub fn open_as_secondary(
    primary: &str,
    secondary: &str,
    option: Option<OptionPy>,
) -> PyResult<DBPy> {
    let mut opts: Options = Options::default();

    if !option.is_none() {
        opts = option.unwrap().inner;
    }

    match DB::open_as_secondary(&opts, primary, secondary) {
        Ok(db) => {
            let db = DBPy {
                db: Some(Arc::new(db)),
                path: secondary.as_bytes().to_vec(),
            };

            return Ok(db);
        }
        Err(e) => Err(RocksDBPyException::new_err(format!(
            "Database cannot be open for read only, {}",
            e
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
