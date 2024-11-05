use thiserror::Error;
use tokio::task::{self};

// Custom errors that catch various errors that can occur in the application.

#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),

    #[error("Failed password verification.")]
    InvalidPassword,
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error(transparent)]
    JoinError(#[from] task::JoinError),
}
