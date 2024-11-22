use track::{error::AppError, feature::cli::Cli, init};
use error_stack::{Report, Result, ResultExt};

fn main() -> Result<(), AppError> {
  init::error_reporting();
  init::tracing();
  Cli::new().display_menu();


  Ok(())
}