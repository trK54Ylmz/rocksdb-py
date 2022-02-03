use pyo3::prelude::*;
use rocksdb::DB;
use std::sync::Arc;

#[pyclass(name="RocksDB")]
pub struct RocksDBPy {
    pub db: Arc<DB>,
    pub path: Vec<u8>,
}

#[pymethods]
impl RocksDBPy {
    
}
