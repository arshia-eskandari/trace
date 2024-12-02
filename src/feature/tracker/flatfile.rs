use std::{ fs::{ self, File, OpenOptions }, io::{ Seek, SeekFrom }, path::{ Path, PathBuf } };
use fs2::FileExt;
use serde::{ Deserialize, Serialize };
use thiserror::Error;
use std::io::{ Read, Write };
use chrono::Local;
use serde_json;
use error_stack::Report;

#[derive(Error, Debug)]
pub enum FlatFileError {
    #[error("timer is already running")]
    ActiveTimer,

    #[error("timer is not running")]
    InactiveTimer,

    #[error("failed to create or open the lock file: {0}")] LockFileError(#[source] std::io::Error),

    #[error("failed to create or open the database file: {0}")] DbFileError(
        #[source] std::io::Error,
    ),

    #[error("failed to read from the database file: {0}")] ReadError(#[source] std::io::Error),

    #[error("failed to write to the database file: {0}")] WriteError(#[source] std::io::Error),

    #[error("failed to parse JSON data: {0}")] JsonParseError(#[source] serde_json::Error),

    #[error("failed to serialize data to JSON: {0}")] JsonSerializeError(
        #[source] serde_json::Error,
    ),
}
pub struct FlatFileTracker {
    db_dir: PathBuf,
    lockfile: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
struct Timestamp(u64, Option<u64>, bool);

impl Timestamp {
    fn new(timstamp: u64, is_active: bool) -> Self {
        Self(timstamp, None, is_active)
    }
}

impl FlatFileTracker {
    pub fn new<D, L>(db_dir: D, lockfile: L) -> Self where D: Into<PathBuf>, L: Into<PathBuf> {
        let db_dir = db_dir.into();
        let lockfile = lockfile.into();
        Self { db_dir, lockfile }
    }

    pub fn start(&self) -> Result<(), Report<FlatFileError>> {
        if self.is_running() {
            return Err(
                Report::new(FlatFileError::ActiveTimer).attach_printable(
                    "a timer is already running"
                )
            );
        }
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.lockfile)
            .map_err(|e|
                Report::new(FlatFileError::ReadError(e)).attach_printable("failed to open lockfile")
            )?
            .lock_exclusive()
            .map_err(|e|
                Report::new(FlatFileError::ReadError(e)).attach_printable(
                    "failed to acquire exclusive lock"
                )
            )?;

        let now = Local::now();
        let timestamp = Timestamp::new(now.timestamp() as u64, true);

        let mut db_file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&self.db_dir)
            .map_err(|e|
                Report::new(FlatFileError::DbFileError(e)).attach_printable(
                    "failed to open db file"
                )
            )?;

        let mut buffer = String::new();
        db_file.read_to_string(&mut buffer).map_err(|e| Report::new(FlatFileError::ReadError(e)))?;

        let data: Vec<Timestamp> = if buffer.trim().is_empty() {
            vec![timestamp]
        } else {
            let mut parsed_data: Vec<Timestamp> = serde_json
                ::from_str(&buffer)
                .map_err(|e| Report::new(FlatFileError::JsonParseError(e)))?;
            let last_index = parsed_data.len() - 1;
            let last_active_option = parsed_data.get_mut(last_index);
            if let Some(last_active_timestamp) = last_active_option {
                last_active_timestamp.2 = false;
            }
            parsed_data.push(timestamp);
            parsed_data
        };

        self.save_file(&mut db_file, data)?;

        Ok(())
    }

    pub fn stop(&self) -> Result<(), Report<FlatFileError>> {
        if !self.is_running() {
            return Err(
                Report::new(FlatFileError::InactiveTimer).attach_printable("timer is not running")
            );
        }

        fs
            ::remove_file(&self.lockfile)
            .map_err(|e|
                Report::new(FlatFileError::LockFileError(e)).attach_printable(
                    "failed to delete lockfile"
                )
            )?;

        
        

        Ok(())
    }

    fn save_file(&self, db_file: &mut File, data: Vec<Timestamp>) -> Result<(), Report<FlatFileError>> {
        db_file
            .set_len(0)
            .map_err(|e|
                Report::new(FlatFileError::DbFileError(e)).attach_printable(
                    "failed to truncate the file"
                )
            )?;
        db_file
            .seek(SeekFrom::Start(0))
            .map_err(|e|
                Report::new(FlatFileError::DbFileError(e)).attach_printable(
                    "failed to find the first line"
                )
            )?;

        let json_data = serde_json
            ::to_string(&data)
            .map_err(|e|
                Report::new(FlatFileError::JsonParseError(e)).attach_printable(
                    "failed to serialize data to JSON"
                )
            )?;
        db_file
            .write_all(json_data.as_bytes())
            .map_err(|e|
                Report::new(FlatFileError::WriteError(e)).attach_printable(
                    "failed to write to database"
                )
            )?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        if Path::new(&self.lockfile).exists() { true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn start_tracking_with_default_tracker() -> Result<(), Report<FlatFileError>> {
        let tracker = FlatFileTracker::new("test_db.json", "test_lockfile");
        tracker.start()?;
        assert!(tracker.is_running());

        Ok(())
    }

    #[test]
    fn stop_tracking_with_default_tracker() -> Result<(), Report<FlatFileError>> {
        let tracker = FlatFileTracker::new("test_db.json", "test_lockfile");
        tracker.start()?;
        thread::sleep(Duration::from_secs(2));
        tracker.stop()?;
        assert!(!tracker.is_running());

        Ok(())
    }
}
