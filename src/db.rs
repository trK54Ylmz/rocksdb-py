use crate::base::*;
use crate::batch::*;
use crate::iterator::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use rocksdb::{Direction, IteratorMode, DB};
use std::sync::Arc;

/// Base RocksDB database.
#[pyclass(name = "RocksDB")]
pub struct DBPy {
    pub path: Vec<u8>,
    pub db: Option<Arc<DB>>,
}

#[pymethods]
impl DBPy {
    /// Return the value associated with a "key".
    ///
    /// # Example
    ///
    /// ```
    /// value = db.get(b'key')
    /// ```
    fn get<'py>(&self, py: Python<'py>, key: &PyBytes) -> PyResult<Option<&'py PyBytes>> {
        if let Some(db) = &self.db {
            match db.get(key.as_bytes()) {
                Ok(None) => Ok(None),
                Ok(Some(value)) => Ok(Some(PyBytes::new(py, &value))),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Record cannot get. {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err("Record cannot get"))
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
        if let Some(db) = &self.db {
            match db.put(key.as_bytes(), value.as_bytes()) {
                Ok(()) => Ok(()),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Record cannot set. {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err("Record cannot set"))
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
        if let Some(db) = &self.db {
            match db.delete(key.as_bytes()) {
                Ok(()) => Ok(()),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Record cannot remove. {}",
                    e
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err("Record cannot remove"))
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

        if let Some(db) = &self.db {
            match db.write(wr) {
                Ok(_) => Ok(()),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Batch cannot write {} elements. {}",
                    len, e,
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err(format!(
                "Batch cannot write {} elements",
                len
            )))
        }
    }

    /// Returns entries according to given list of key and values.
    ///
    /// # Example
    ///
    /// ```
    /// db.multi_get(b'first', b'second')
    ///
    /// db.multi_get(b'first', b'second', skip_missings=True)
    /// ```
    fn multi_get<'py>(
        &mut self,
        py: Python<'py>,
        keys: &'py PyList,
        skip_missings: Option<bool>,
    ) -> PyResult<&'py PyList> {
        // generate list of keys based on Python's list
        let ks: Vec<&[u8]> = keys
            .iter()
            .map(|k| <PyBytes as PyTryFrom>::try_from(k).unwrap().as_bytes())
            .collect();

        let r = PyList::empty(py);
        let skip = skip_missings.is_none() || skip_missings.unwrap() == false;

        if let Some(db) = &self.db {
            for value in db.multi_get(ks) {
                match value {
                    Ok(v) => match v {
                        Some(item) => r.append(PyBytes::new(py, item.as_ref())).unwrap(),
                        None => {
                            // skip missing records if skip_missings is true, the output
                            // array will be shorter then given key array size.
                            if skip {
                                r.append(py.None()).unwrap()
                            } else {
                                continue;
                            }
                        }
                    },
                    Err(e) => {
                        return Err(RocksDBPyException::new_err(format!(
                            "Record cannot get. {}",
                            e,
                        )))
                    }
                }
            }
        }

        Ok(r)
    }

    /// Returns a heap-allocated iterator over the contents of the database.
    ///
    /// # Example
    ///
    /// ```
    /// iterator = db.iterator()
    ///
    /// iterator = db.iterator(mode='end')
    ///
    /// iterator = db.iterator(mode='from', key=b'test')
    ///
    /// iterator = db.iterator(mode='from', key=b'test', direction=-1)
    /// ```
    fn iterator(
        &self,
        mode: Option<&str>,
        key: Option<&PyBytes>,
        direction: Option<i32>,
    ) -> PyResult<IteratorPy> {
        let mut im = IteratorMode::Start;

        if !mode.is_none() {
            let mut ik: &[u8] = b"";
            let mut dr = Direction::Forward;

            if !key.is_none() {
                ik = key.unwrap().as_bytes();
            }

            // Generate direction by minus or plus integer
            if !key.is_none() && !direction.is_none() {
                dr = match direction.unwrap() {
                    -1 => Direction::Reverse,
                    _ => Direction::Forward,
                }
            }

            im = match mode.unwrap() {
                "end" => IteratorMode::End,
                "from" => IteratorMode::From(ik, dr),
                _ => IteratorMode::Start,
            }
        }

        if let Some(db) = &self.db {
            Ok(IteratorPy::new(db.as_ref(), im))
        } else {
            Err(RocksDBPyException::new_err("Iterator cannot get"))
        }
    }

    /// Request stopping background work, if wait is true wait until it’s done.
    ///
    /// # Example
    ///
    /// ```
    /// db.cancel_all_background_work()
    ///
    /// db.cancel_all_background_work(True)
    /// ```
    fn cancel_all_background_work(&self, wait: Option<bool>) -> PyResult<()> {
        let mut w = false;

        if wait.is_some() {
            w = wait.unwrap()
        }

        if let Some(db) = &self.db {
            db.cancel_all_background_work(w);

            Ok(())
        } else {
            Err(RocksDBPyException::new_err("Cancel cannot do"))
        }
    }

    /// Flushes database memtables to SST files on the disk using default options.
    ///
    /// # Example
    ///
    /// ```
    /// db.flush()
    /// ```
    fn flush(&self) -> PyResult<()> {
        if let Some(db) = &self.db {
            match db.flush() {
                Ok(_) => Ok(()),
                Err(e) => Err(RocksDBPyException::new_err(format!(
                    "Database cannot flush. {}",
                    e,
                ))),
            }
        } else {
            Err(RocksDBPyException::new_err("Database cannot flush"))
        }
    }

    /// Close active database
    ///
    /// # Example
    ///
    /// ```
    /// db.close()
    /// ```
    fn close(&mut self) -> PyResult<()> {
        self.db = None;

        Ok(())
    }
}
