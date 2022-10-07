use pyo3::prelude::{pyclass, pymethods};

#[derive(Clone)]
#[pyclass]
pub struct Config;

#[pymethods]
impl Config {
    #[new]
    fn new() -> Self {
        Config
    }
}
