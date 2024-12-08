use track::{ error::AppError, init };
use error_stack::{ Report, Result, ResultExt };

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();
    init
        ::cli_init()
        .map_err(|e| e.change_context(AppError).attach_printable("failed to initialize cli"))?;

    Ok(())
}
