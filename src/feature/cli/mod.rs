pub struct Cli;
use crate::feature::string_ext::StringExt;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum CliCommand {
    Start,
    Stop,
    Report,
}

#[derive(Debug, Copy, Clone)]
pub enum CliFlag {
    DBDirectory,
    Lockfile,
    Verbose,
    Quiet,
    Help,
    Version,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid cli option: {0}")]
pub struct InvalidCliFlag(String);
impl TryFrom<&str> for CliFlag {
    type Error = InvalidCliFlag;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: String = value.into();
        match value.as_str() {
            "-d" | "--db-dir" => Ok(CliFlag::DBDirectory),
            "-l" | "--lockfile" => Ok(CliFlag::Lockfile),
            "-v" | "--verbose" => Ok(CliFlag::Verbose),
            "-q" | "--quiet" => Ok(CliFlag::Quiet),
            "-h" | "--help" => Ok(CliFlag::Help),
            "-V" | "--version" => Ok(CliFlag::Version),
            _ => Err(InvalidCliFlag(value.to_string())),
        }
    }
}

impl Cli {
    pub fn new() -> Self {
        Self {}
    }

    pub fn display_menu(&self) {
        println!("Usage: track [OPTIONS] <COMMAND>");
        println!("");
        println!("Commands:");
        println!(
            "{} {}",
            "start".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Start tracking time"
        );
        println!(
            "{} {}",
            "stop".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Stop tracking time"
        );
        println!(
            "{} {}",
            "report".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Report tracked time for the last 24 hours"
        );
        println!(
            "{} {}",
            "help".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Print this message or the help of the given subcommand(s)"
        );
        println!("");
        println!("Options");
        println!(
            "{} {}",
            "-d, --db-dir <DB_DIR>".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Path to database file"
        );
        println!(
            "{} {}",
            "-l, --lockfile <LOCKFILE>".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Path to lockfile"
        );
        println!(
            "{} {}",
            "-v, --verbose...".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Increase logging verbosity"
        );
        println!(
            "{} {}",
            "-q, --quite...".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Decrease logging verbosity"
        );
        println!(
            "{} {}",
            "-h, --help".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Print help"
        );
        println!(
            "{} {}",
            "-V, --version".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Print Version"
        );
    }
}
