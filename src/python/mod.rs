mod config;
mod error;
use pyo3::exceptions::PyValueError;
use pyo3::panic::PanicException;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyString};
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rgym(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_class()
    // m.add_wrapped(wrapper)
    Ok(())
}

pub struct Gym {}
#[pyclass]
pub struct PyGym {
    pub env: Gym,
}

impl PyGym {
    pub fn new(env: Gym) -> Self {
        PyGym { env }
    }
}
pub struct GymConfig {}
impl GymConfig {
    fn build() -> Gym {
        Gym {}
    }
}

#[pymethods]
impl PyGym {
    #[new]
    pub fn __init__() -> PyResult<Self> {
        Ok(PyGym { env: Gym {} })
    }
}
