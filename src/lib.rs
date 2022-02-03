mod base;
mod batch;
mod db;
mod option;

use crate::base::*;
use crate::batch::*;
use crate::db::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn rocksdbpy(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RocksDBPy>()?;
    m.add_class::<WriteBatchPy>()?;

    m.add_function(wrap_pyfunction!(open, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_default, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_with_ttl, m)?).unwrap();
    m.add_function(wrap_pyfunction!(destroy, m)?).unwrap();

    m.add("RocksDBPyException", py.get_type::<RocksDBPyException>())?;

    Ok(())
}
