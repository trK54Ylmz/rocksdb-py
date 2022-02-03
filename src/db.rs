use crate::base::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use rocksdb::DB;
use std::sync::Arc;

#[pyclass(name = "RocksDB")]
pub struct RocksDBPy {
    pub db: Arc<DB>,
    pub path: Vec<u8>,
}

#[pymethods]
impl RocksDBPy {
    /// Return the value associated with a "key".
    ///
    /// # Example
    ///
    /// ```
    /// value = db.get(b'key')
    /// ```
    fn get<'py>(&mut self, py: Python<'py>, key: &PyBytes) -> PyResult<Option<&'py PyBytes>> {
        match self.db.get(key.as_bytes()) {
            Ok(None) => Ok(None),
            Ok(Some(value)) => Ok(Some(PyBytes::new(py, &value))),
            Err(e) => Err(RocksDBPyException::new_err(format!(
                "Record cannot get. {}",
                e
            ))),
        }
    }

    /// Sets records by "key" and "value".
    ///
    /// # Example
    ///
    /// ```
    /// db.set(b'key', b'value')
    /// ```
    fn set(&mut self, key: &PyBytes, value: &PyBytes) -> PyResult<()> {
        match self.db.put(key.as_bytes(), value.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => Err(RocksDBPyException::new_err(format!(
                "Record cannot set. {}",
                e
            ))),
        }
    }

    /// Removes existing records by "key".
    ///
    /// # Example
    ///
    /// ```
    /// db.delete(b'key')
    /// ```
    fn delete(&mut self, key: &PyBytes) -> PyResult<()> {
        match self.db.delete(key.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => Err(RocksDBPyException::new_err(format!(
                "Record cannot remove. {}",
                e
            ))),
        }
    }
}
