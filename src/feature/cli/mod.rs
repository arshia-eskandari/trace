pub struct CLI;
use crate::feature::string_ext::StringExt;

impl CLI {
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
            "-v, --version".to_string().pad_start(2, None).pad_end_to_length(8, None),
            "Print Version"
        );
    }
}
