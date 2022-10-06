#![allow(unused_imports)]
use anyhow::Error;
use pyo3::{create_exception, exceptions, PyErr};
create_exception!(exceptions, NotFoundError, pyo3::exceptions::PyException);
create_exception!(exceptions, ComputeError, pyo3::exceptions::PyException);
create_exception!(exceptions, NoDataError, pyo3::exceptions::PyException);
create_exception!(exceptions, ShapeError, pyo3::exceptions::PyException);
create_exception!(exceptions, SchemaError, pyo3::exceptions::PyException);
create_exception!(exceptions, DuplicateError, pyo3::exceptions::PyException);
create_exception!(
    exceptions,
    InvalidOperationError,
    pyo3::exceptions::PyException
);

use thiserror::Error as ThisError;
#[derive(Debug)]
pub enum ErrString {
    Owned(String),
    Borrowed(&'static str),
}

impl From<&'static str> for ErrString {
    fn from(msg: &'static str) -> Self {
        if std::env::var("POLARS_PANIC_ON_ERR").is_ok() {
            panic!("{}", msg)
        } else {
            ErrString::Borrowed(msg)
        }
    }
}

impl From<String> for ErrString {
    fn from(msg: String) -> Self {
        if std::env::var("POLARS_PANIC_ON_ERR").is_ok() {
            panic!("{}", msg)
        } else {
            ErrString::Owned(msg)
        }
    }
}

impl std::fmt::Display for ErrString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ErrString::Owned(msg) => msg.as_str(),
            ErrString::Borrowed(msg) => msg,
        };
        write!(f, "{}", msg)
    }
}
#[derive(Debug, ThisError)]
pub enum GymError {
    #[error("Invalid operation {0}")]
    InvalidOperation(ErrString),
    #[error("Data types don't match: {0}")]
    SchemaMisMatch(ErrString),
    #[error("Not found: {0}")]
    NotFound(ErrString),
    #[error("Lengths don't match: {0}")]
    ShapeMisMatch(ErrString),
    #[error("{0}")]
    ComputeError(ErrString),
    #[error("Such empty...: {0}")]
    NoData(ErrString),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("DuplicateError: {0}")]
    Duplicate(ErrString),
}

pub type GymResult<T> = std::result::Result<T, GymError>;

#[derive(ThisError)]
pub enum PyGymErr {
    #[error(transparent)]
    Gym(#[from] GymError),
}
impl std::fmt::Debug for PyGymErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PyGymErr::*;
        match self {
            Gym(err) => write!(f, "{:?}", err),
        }
    }
}
impl std::convert::From<PyGymErr> for PyErr {
    fn from(err: PyGymErr) -> PyErr {
        match &err {
            PyGymErr::Gym(err) => match err {
                GymError::NotFound(name) => NotFoundError::new_err(name.to_string()),
                GymError::ComputeError(err) => ComputeError::new_err(err.to_string()),
                GymError::NoData(err) => NoDataError::new_err(err.to_string()),
                GymError::ShapeMisMatch(err) => ShapeError::new_err(err.to_string()),
                GymError::SchemaMisMatch(err) => SchemaError::new_err(err.to_string()),
                GymError::Io(err) => pyo3::exceptions::PyIOError::new_err(err.to_string()),
                GymError::Duplicate(err) => DuplicateError::new_err(err.to_string()),
                GymError::InvalidOperation(err) => InvalidOperationError::new_err(err.to_string()),
            },
            _ => pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", &err)),
        }
    }
}
