mod base;
mod batch;
mod db;
mod iterator;
mod option;

use crate::base::*;
use crate::batch::*;
use crate::db::*;
use crate::iterator::*;
use crate::option::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn rocksdbpy(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DBPy>()?;
    m.add_class::<IteratorPy>()?;
    m.add_class::<OptionPy>()?;
    m.add_class::<WriteBatchPy>()?;

    m.add_function(wrap_pyfunction!(open, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_default, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_with_ttl, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_for_readonly, m)?).unwrap();
    m.add_function(wrap_pyfunction!(open_as_secondary, m)?).unwrap();
    m.add_function(wrap_pyfunction!(destroy, m)?).unwrap();

    m.add("RocksDBException", py.get_type::<RocksDBPyException>())?;

    Ok(())
}
