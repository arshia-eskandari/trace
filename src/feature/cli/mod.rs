use clap::{ command, Parser, Subcommand };
use error_stack::Report;

use super::tracker::flatfile::{ FlatFileError, FlatFileTracker };

#[derive(thiserror::Error, Debug)]
#[error("a cli error occured")]
pub struct CliError;

#[derive(Debug, Parser)]
#[command(
    name = "track",
    version = "1.0",
    about = "Time Tracker Application",
    arg_required_else_help(true)
)]
struct Cli {
    /// Path to database file
    #[arg(short, long, global = true)]
    db_dir: Option<String>,

    /// Path to lockfile
    #[arg(short, long, global = true)]
    lockfile: Option<String>,

    /// Increase logging verbosity
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Decrease logging verbosity
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    quiet: u8,

    /// Subcommands for specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start tracking time
    Start,

    /// Stop tracking time
    Stop,

    /// Report tracked time for the last 24 hours
    Report,
}

pub fn init() -> Result<(), Report<CliError>> {
    let cli = Cli::parse();
    let Cli { db_dir: db_dir_option, lockfile: lock_file_option, verbose, quiet, .. } = cli;
    let db_dir = db_dir_option.unwrap_or("db.json".to_owned());
    let db_dir = if db_dir.ends_with(".json") { db_dir } else { format!("{}.json", db_dir) };
    let lockfile = lock_file_option.unwrap_or("lockfile".to_owned());
    let tracker = FlatFileTracker::new(&db_dir, &lockfile);
    let verbosity = if quiet > 0 { -1 as i8 } else { verbose as i8 };
    let handle_start = || -> Result<(), Report<CliError>> {
        tracker
            .start(verbosity)
            .map_err(|e| e.change_context(CliError).attach_printable("tracker failed to start"))?;

        Ok(())
    };
    let handle_stop = || -> Result<(), Report<CliError>> {
        tracker
            .stop(verbosity)
            .map_err(|e| e.change_context(CliError).attach_printable("tracker failed to stop"))?;
        Ok(())
    };
    let handle_report = || -> Result<(), Report<CliError>> {
        tracker
            .report(verbosity)
            .map_err(|e| e.change_context(CliError).attach_printable("tracker failed to report"))?;
        Ok(())
    };
    match cli.command {
        Commands::Start => handle_start()?,
        Commands::Stop => handle_stop()?,
        Commands::Report => handle_report()?,
    }

    Ok(())
}
