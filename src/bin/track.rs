use track::{error::AppError, feature::cli::CLI, init};
use error_stack::{Report, Result, ResultExt};

fn main() -> Result<(), AppError> {
  init::error_reporting();
  init::tracing();
  CLI::new().display_menu();


  Ok(())
}