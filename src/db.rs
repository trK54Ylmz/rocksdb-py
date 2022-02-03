use crate::base::*;
use crate::batch::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use rocksdb::DB;
use std::sync::Arc;

/// Base RocksDB database.
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

    /// Sets database entries for list of key and values as a batch.
    ///
    /// # Example
    ///
    /// ```
    /// b = WriteBatch()
    /// b.add(b'first', 'first_value')
    /// b.add(b'second', 'second_value')
    ///
    /// db.write(b)
    /// ```
    fn write(&self, batch: &mut WriteBatchPy) -> PyResult<()> {
        let wr = batch.get().unwrap();
        let len = wr.len();

        match self.db.write(wr) {
            Ok(_) => Ok(()),
            Err(e) => Err(RocksDBPyException::new_err(format!(
                "Batch cannot write {} elements. {}",
                len, e,
            ))),
        }
    }
}
