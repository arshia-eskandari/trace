use std::path::PathBuf;

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

    pub fn start(&self) {}

    pub fn is_running(&self) -> bool {
        false
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
