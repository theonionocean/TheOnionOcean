#[derive(Debug)]
pub struct ValidationError {
    pub field: &'static str,
    pub error_message: String,
    pub code: String,
}

impl std::error::Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.code, self.field, self.error_message)
    }
}
