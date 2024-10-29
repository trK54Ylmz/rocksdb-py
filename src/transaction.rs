use pyo3::{prelude::*, types::PyBytes};
use rocksdb::{Transaction, TransactionDB};

use crate::RocksDBPyException;

#[pyclass(name = "Transaction")]
pub struct TransactionPy {
    inner: Option<Transaction<'static, TransactionDB>>,
}

#[pymethods]
impl TransactionPy {
    /// Put the key value in default column family and do conflict checking on the key.
    ///
    /// # Example
    ///
    /// ```
    /// tx.add('test', '1')
    /// tx.add('test', '1 2 3')
    /// ```
    pub fn add(&mut self, key: &PyBytes, value: &PyBytes) -> PyResult<()> {
        if let Some(inner) = &mut self.inner {
            match inner.put(key.as_bytes(), value.as_bytes()) {
                Ok(ok) => Ok(ok),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Transaction append operation is failed, {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err(
                "Transaction is invalid. New transaction is required",
            ))
        }
    }

    /// Merge value with existing value of key, and also do conflict checking on the key.
    ///
    /// # Example
    ///
    /// ```
    /// tx.merge('test', '2 3')
    /// ```
    pub fn merge(&mut self, key: &PyBytes, value: &PyBytes) -> PyResult<()> {
        if let Some(inner) = &mut self.inner {
            match inner.merge(key.as_bytes(), value.as_bytes()) {
                Ok(ok) => Ok(ok),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Transaction merge operation is failed, {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err(
                "Transaction is invalid. New transaction is required",
            ))
        }
    }

    /// Write all batched keys to the DB atomically.
    ///
    /// # Example
    ///
    /// ```
    /// tx.commit()
    /// ```
    pub fn commit(&mut self) -> PyResult<()> {
        println!("test");
        if let Some(inner) = self.inner.take() {
            println!("test 2");

            match inner.commit() {
                Ok(ok) => Ok(ok),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Transaction merge operation is failed, {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err(
                "Transaction is invalid. New transaction is required",
            ))
        }
    }
}

impl TransactionPy {
    pub fn new(db: &TransactionDB) -> TransactionPy {
        unsafe {
            TransactionPy {
                inner: Some(std::mem::transmute(db.transaction())),
            }
        }
    }
}
