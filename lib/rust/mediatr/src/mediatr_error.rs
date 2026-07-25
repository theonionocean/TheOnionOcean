#[derive(Debug)]
pub enum MediatrError {
    CommandNotFound(String),
    QueryNotFound(String),
    HandlerNotFound(String),
    HandlerFailed(String),
}

impl std::error::Error for MediatrError {}

impl std::fmt::Display for MediatrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediatrError::CommandNotFound(command) => write!(f, "Command not found: {}", command),
            MediatrError::QueryNotFound(query) => write!(f, "Query not found: {}", query),
            MediatrError::HandlerNotFound(handler) => write!(f, "Handler not found: {}", handler),
            MediatrError::HandlerFailed(message) => write!(f, "Handler failed: {}", message),
        }
    }
}
