use thiserror;

#[derive(Debug, thiserror::Error)]
#[error("an application error occured")]
pub struct AppError;

#[derive(thiserror::Error, Debug)]
#[error("a cli error occured")]
pub struct CliErrorTest;

