use clap::{ command, Parser, Subcommand };

#[derive(Debug, Parser)]
#[command(name = "track", version = "1.0", about = "Time Tracker Application", arg_required_else_help(true))]
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

pub fn init() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => handle_start(cli.db_dir, cli.lockfile, cli.verbose, cli.quiet),
        Commands::Stop => handle_stop(cli.db_dir, cli.lockfile, cli.verbose, cli.quiet),
        Commands::Report => handle_report(cli.db_dir, cli.lockfile, cli.verbose, cli.quiet),
    }
}

fn handle_start(db_dir: Option<String>, lockfile: Option<String>, verbose: u8, quiet: u8) {
    println!(
        "Stating with {}, {}, {}, and {}",
        db_dir.unwrap_or("no db".to_string()),
        lockfile.unwrap_or("no lockfile".to_string()),
        verbose,
        quiet
    )
}

fn handle_stop(db_dir: Option<String>, lockfile: Option<String>, verbose: u8, quiet: u8) {
    println!(
        "Stoping with {}, {}, {}, and {}",
        db_dir.unwrap_or("no db".to_string()),
        lockfile.unwrap_or("no lockfile".to_string()),
        verbose,
        quiet
    )
}

fn handle_report(db_dir: Option<String>, lockfile: Option<String>, verbose: u8, quiet: u8) {
    println!(
        "Reporting with {}, {}, {}, and {}",
        db_dir.unwrap_or("no db".to_string()),
        lockfile.unwrap_or("no lockfile".to_string()),
        verbose,
        quiet
    )
}
