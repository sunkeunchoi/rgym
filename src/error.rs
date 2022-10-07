#![allow(unused_imports)]
use crate::{build_error, to_pyerror};
use anyhow::Error;

use pyo3::{create_exception, exceptions, PyErr};
#[derive(Debug)]
pub struct InvalidOperationError;

impl error_stack::Context for InvalidOperationError {}
impl std::fmt::Display for InvalidOperationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("Invalid operation")
    }
}

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
