use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("cannot init database pool : {0}")]
    Pool(String),
    #[error("cannot find environment variable for database init : {0}")]
    Variable(String),
    #[error("database init error : {0}")]
    GenericInit(String),
}
