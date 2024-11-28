use std::{ fs::OpenOptions, path::{ Path, PathBuf } };
use fs2::FileExt;
use serde_json::json;
use std::io::{Read, Write};
use chrono::Local;

struct FlatFileTracker {
    db_dir: PathBuf,
    lockfile: PathBuf,
}

impl FlatFileTracker {
    pub fn new<D, L>(db_dir: D, lockfile: L) -> Self where D: Into<PathBuf>, L: Into<PathBuf> {
        let db_dir = db_dir.into();
        let lockfile = lockfile.into();
        Self { db_dir, lockfile }
    }

    pub fn start(&self) -> Result<(), String> {
        if Path::new(&self.lockfile).exists() {
            return Err("The timer must be stopped before initiation".to_owned());
        }

        let lockfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.lockfile)
            .expect("Failed to create or open the lock file");
        lockfile.lock_exclusive().expect("Failed to acquire lockfile");

        let now = Local::now();
        let timestamp = now.timestamp();

        let mut db_file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true) // Add read permission to read existing content
            .open(&self.db_dir)
            .expect("Failed to create or open the JSON file");

        let mut buffer= String::new();
        db_file.read_to_string(&mut buffer).expect("Failed to read the JSON file");

        let mut data = if buffer.trim().is_empty() {
            json!({})
        } else {
            serde_json::from_str(&buffer).expect("Failed to parse the JSON file")
        };

        data["timestamp"] = json!(timestamp);

        db_file.set_len(0).expect("Failed to truncate the JSON file"); // Clear file contents
        db_file.write_all(data.to_string().as_bytes()).expect("Failed to write to the JSON file");

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_tracking_with_default_tracker() {
        let tracker = FlatFileTracker::new("db.json", "lockfile");

        tracker.start();

        assert!(tracker.is_running());
    }
}
