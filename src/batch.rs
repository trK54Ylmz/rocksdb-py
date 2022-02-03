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
        if let Some(inner) = &mut self.writer {
            Ok(inner.put(key.as_bytes(), value.as_bytes()))
        } else {
            Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            ))
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
        if let Some(inner) = &mut self.writer {
            Ok(inner.delete(key.as_bytes()))
        } else {
            Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            ))
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
        if let Some(inner) = &mut self.writer {
            Ok(inner.clear())
        } else {
            Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            ))
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
        if let Some(inner) = &mut self.writer {
            Ok(inner.len())
        } else {
            Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            ))
        }
    }
}

impl WriteBatchPy {
    pub fn get(&mut self) -> PyResult<WriteBatch> {
        if let Some(inner) = self.writer.take() {
            Ok(inner)
        } else {
            Err(RocksDBPyException::new_err(
                "Batch writer is invalid. New writer is required",
            ))
        }
    }
}
