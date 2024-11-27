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
        // Check if the lockfile exists
        if Path::new(&self.lockfile).exists() {
            return Err("The timer must be stopped before initiation".to_owned());
        }

        // Create or open the lockfile
        let lockfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.lockfile)
            .expect("Failed to create or open the lock file");
        lockfile.lock_exclusive().expect("Failed to acquire lockfile");

        // Get the current timestamp
        let now = Local::now();
        let timestamp = now.timestamp();

        // Create or open the JSON file (db_dir)
        let mut db_file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true) // Add read permission to read existing content
            .open(&self.db_dir)
            .expect("Failed to create or open the JSON file");

        // Read existing JSON data, if any
        let mut db_content = String::new();
        db_file.read_to_string(&mut db_content).expect("Failed to read the JSON file");

        let mut data = if db_content.trim().is_empty() {
            // If file is empty, start with an empty JSON object
            json!({})
        } else {
            // Parse existing JSON data
            serde_json::from_str(&db_content).expect("Failed to parse the JSON file")
        };

        // Add or update the `timestamp` field
        data["timestamp"] = json!(timestamp);

        // Write the updated JSON data back to the file
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
