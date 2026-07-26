#[derive(Debug)]
pub enum SurrealDbError {
    RecordNotFound(String),
    RecordAlreadyExists(String),
    RecordQueryError(String),
    InternalError(String),
}

impl std::error::Error for SurrealDbError {}

impl std::fmt::Display for SurrealDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SurrealDbError::RecordNotFound(record) => write!(f, "Record not found: {}", record),
            SurrealDbError::RecordAlreadyExists(record) => {
                write!(f, "Record already exists: {}", record)
            }
            SurrealDbError::RecordQueryError(record) => write!(f, "Record not created: {}", record),
            SurrealDbError::InternalError(error) => write!(f, "Internal error: {}", error),
        }
    }
}

impl From<surrealdb::Error> for SurrealDbError {
    fn from(error: surrealdb::Error) -> Self {
        // error.
        match error.kind_str() {
            "NotFound" => SurrealDbError::RecordNotFound(error.message().to_string()),
            "AlreadyExists" => SurrealDbError::RecordAlreadyExists(error.message().to_string()),
            "Query" => SurrealDbError::RecordQueryError(error.message().to_string()),
            _ => SurrealDbError::InternalError(error.to_string()),
        }
    }
}
