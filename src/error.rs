use thiserror;

#[derive(Debug, thiserror::Error)]
#[error("an application error occured")]
pub struct AppError;