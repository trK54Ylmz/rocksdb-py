use pyo3::prelude::*;
use rocksdb::Options;

#[pyclass(name="Option")]
#[derive(Clone)]
pub struct OptionPy {
    pub inner: Options,
}

#[pymethods]
impl OptionPy {
    #[new]
    pub fn new() -> Self {
        let opts = Options::default();

        OptionPy { inner: opts }
    }

    /// If true, the database will be created if it is missing.
    /// 
    /// Default: `false`
    /// 
    /// # Example
    /// 
    /// ````
    /// opts.create_if_missing(True)
    /// ```
    pub fn create_if_missing(&mut self, create_if_missing: bool) {
        self.inner.create_if_missing(create_if_missing);
    }
}
