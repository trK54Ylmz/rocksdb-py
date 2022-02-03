use crate::base::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use rocksdb::WriteBatch;

/// Batch writer.
#[pyclass(name = "WriteBatch")]
pub struct WriteBatchPy {
    writer: Option<WriteBatch>,
}

#[pymethods]
impl WriteBatchPy {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(WriteBatchPy {
            writer: Some(WriteBatch::default()),
        })
    }

    /// Append new "key" and "value" in the batch.
    ///
    /// # Example
    ///
    /// ```
    /// b = WriteBatch()
    ///
    /// b.add(b'first', 'first_value')
    /// ```
    fn add(&mut self, key: &PyBytes, value: &PyBytes) -> PyResult<()> {
        match &mut self.writer {
            Some(inner) => Ok(inner.put(key.as_bytes(), value.as_bytes())),
            None => Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            )),
        }
    }

    /// Remove "key" from the batch.
    ///
    /// # Example
    ///
    /// ```
    /// b = WriteBatch()
    ///
    /// b.delete(b'first')
    /// ```
    fn delete(&mut self, key: &PyBytes) -> PyResult<()> {
        match &mut self.writer {
            Some(inner) => Ok(inner.delete(key.as_bytes())),
            None => Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            )),
        }
    }

    /// Clear the batch.
    ///
    /// # Example
    ///
    /// ```
    /// b = WriteBatch()
    ///
    /// b.clear()
    /// ```
    fn clear(&mut self) -> PyResult<()> {
        match &mut self.writer {
            Some(inner) => Ok(inner.clear()),
            None => Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            )),
        }
    }

    /// Returns element count of the batch.
    ///
    /// # Example
    ///
    /// ```
    /// b = WriteBatch()
    ///
    /// size = b.len()
    /// ```
    fn len(&mut self) -> PyResult<usize> {
        match &mut self.writer {
            Some(inner) => Ok(inner.len()),
            None => Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            )),
        }
    }
}

impl WriteBatchPy {
    #[inline]
    pub fn get(&mut self) -> PyResult<WriteBatch> {
        match self.writer.take() {
            Some(inner) => Ok(inner),
            None => Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            )),
        }
    }
}
