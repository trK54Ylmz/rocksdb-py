mod base;
mod db;
mod option;

use crate::base::*;
use crate::db::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn rocksdbpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RocksDBPy>()?;

    m.add_function(wrap_pyfunction!(open, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_default, m)?).unwrap();
    m.add_function(wrap_pyfunction!(destroy, m)?).unwrap();

    Ok(())
}
