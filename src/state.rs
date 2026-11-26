//! The cat state that all terminals share (ADR-0004): the time of the last meal.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// `$XDG_STATE_HOME/meow/state`, or `~/.local/state/meow/state`.
pub fn path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))?;
    Some(base.join("meow").join("state"))
}

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// Time of the last meal (Unix seconds). `None` if the cat was never fed or the file is not valid.
pub fn fed_at(path: &Path) -> Option<u64> {
    fs::read_to_string(path).ok()?.lines().find_map(|l| l.strip_prefix("fed_at=")?.trim().parse().ok())
}

/// Writes the meal time atomically: other terminals see the old file or the new file, never a part.
pub fn save_fed_at(path: &Path, t: u64) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("tmp.{}", std::process::id()));
    fs::write(&tmp, format!("fed_at={t}\n"))?;
    fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meal_time_survives_a_round_trip() {
        let dir = std::env::temp_dir().join(format!("meow-test-{}", std::process::id()));
        let file = dir.join("meow").join("state");
        assert_eq!(fed_at(&file), None);
        save_fed_at(&file, 1_764_160_000).unwrap();
        assert_eq!(fed_at(&file), Some(1_764_160_000));
        fs::write(&file, "fed_at=not a number\n").unwrap();
        assert_eq!(fed_at(&file), None);
        fs::remove_dir_all(dir).unwrap();
    }
}
