use crate::base::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use rocksdb::{DBIterator, IteratorMode, DB};

/// DB iterator.
#[pyclass(name = "DBIterator")]
pub struct IteratorPy {
    inner: Option<DBIterator<'static>>,
}

#[pymethods]
impl IteratorPy {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    /// Returns next database entry.
    ///
    /// # Example
    ///
    /// ```
    /// next(iterator)
    /// ```
    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        if let Some(inner) = &mut slf.inner {
            match inner.next() {
                None => Ok(None),
                Some(kv) => {
                    match kv {
                        Ok(kv) => {
                            let py = slf.py();
                            let key = PyBytes::new(py, kv.0.as_ref());
                            let value = PyBytes::new(py, kv.1.as_ref());
                            return Ok(Some(PyTuple::new(py, &[key, value]).into_py(py)));
                        },
                        Err(_err) => {
                            Err(RocksDBPyException::new_err("Iterator completed?"))
                        }
                    }
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Returns element count of the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// len(itr)
    /// ```
    fn __len__(mut slf: PyRefMut<Self>) -> PyResult<usize> {
        if let Some(inner) = &mut slf.inner {
            Ok(inner.count())
        } else {
            Err(RocksDBPyException::new_err("Length cannot get"))
        }
    }

    /// Returns element count of the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// count = itr.len()
    /// ```
    fn len(mut slf: PyRefMut<Self>) -> PyResult<usize> {
        if let Some(inner) = &mut slf.inner {
            Ok(inner.count())
        } else {
            Err(RocksDBPyException::new_err("Count cannot get"))
        }
    }

    /// Close and destroy active iterator
    ///
    /// # Example
    ///
    /// ```
    /// itr.close()
    /// ```
    fn close(mut slf: PyRefMut<Self>) -> PyResult<()> {
        slf.inner = None;

        Ok(())
    }
}

impl IteratorPy {
    pub fn new(db: &DB, mode: IteratorMode) -> IteratorPy {
        unsafe {
            IteratorPy {
                inner: Some(std::mem::transmute(db.iterator(mode))),
            }
        }
    }
}

impl Drop for IteratorPy {
    fn drop(&mut self) {
        self.inner = None
    }
}
