use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use rocksdb::{DBIterator, IteratorMode, DB};

/// DB iterator.
#[pyclass(name = "DBIterator")]
pub struct RocksDBIteratorPy {
    inner: DBIterator<'static>,
}

#[pymethods]
impl RocksDBIteratorPy {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        for (k, v) in slf.inner.next() {
            let py = slf.py();
            let key = PyBytes::new(py, k.as_ref());
            let value = PyBytes::new(py, v.as_ref());

            return Ok(Some(PyTuple::new(py, &[key, value]).into_py(py)));
        }

        Ok(None)
    }
}

impl RocksDBIteratorPy {
    pub fn new(db: &DB, mode: IteratorMode) -> RocksDBIteratorPy {
        unsafe {
            RocksDBIteratorPy {
                inner: std::mem::transmute(db.iterator(mode)),
            }
        }
    }
}
