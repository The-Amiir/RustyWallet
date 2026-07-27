use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not logged in")]
    NotLoggedIn,
    #[error("User already exists")]
    UserExists,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;