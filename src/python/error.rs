#![allow(unused_imports)]
use crate::error::InvalidOperationError;
use pyo3::{create_exception, exceptions, PyErr};
// create python error object from core error structs
create_exception!(
    exceptions,
    InvalidOperationException,
    pyo3::exceptions::PyException
);
// impl std::convert::From to match python error object
impl std::convert::From<InvalidOperationError> for PyErr {
    fn from(err: InvalidOperationError) -> PyErr {
        InvalidOperationException::new_err(err.to_string())
    }
}
