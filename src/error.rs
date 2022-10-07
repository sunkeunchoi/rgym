#[derive(Debug)]
pub struct InvalidOperationError;

impl error_stack::Context for InvalidOperationError {}
impl std::fmt::Display for InvalidOperationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("Invalid operation")
    }
}
