#[macro_export]
macro_rules! build_error {
    ($t:ty, $write:expr) => {
        impl error_stack::Context for $t {}
        impl std::fmt::Display for $t {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fmt.write_str($write)
            }
        }
    };
}

#[macro_export]
macro_rules! to_pyerror {
    ($from:ty, $to:ty) => {
        impl std::convert::From<$from> for pyo3::PyErr {
            fn from(err: $from) -> pyo3::PyErr {
                <$to>::new_err(err.to_string())
            }
        }
    };
}
