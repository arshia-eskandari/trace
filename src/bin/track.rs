use track::{error::AppError, init};
use error_stack::{Report, Result, ResultExt};

fn main() -> Result<(), AppError> {
  init::error_reporting();
  init::tracing();
  init::cli_init();

  Ok(())
}