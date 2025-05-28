use serde::Serialize;

pub type Result<T> = core::result::Result<T, DatabaseError>;

#[derive(Debug, Serialize)]
pub enum DatabaseError { // To be improved later
    LoginFail,
    ConnectionError(String),
    SchemaError(String),
    QueryError(String),
    // etc.
    Multiple(Vec<DatabaseError>)
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for DatabaseError {}
