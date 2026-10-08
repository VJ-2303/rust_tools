use anyhow::{Ok, Result};
use std::fs::{self};
use std::io::Write;
use std::path::Path;

pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
    if fs::exists(&path)? {
        let text = fs::read_to_string(&path)?;
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}

pub fn append(path: impl AsRef<Path>, msg: &str) -> Result<()> {
    let mut logbook = fs::File::options().create(true).append(true).open(&path)?;
    writeln!(logbook, "{msg}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let text = read("tests/data/notfound.txt").unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn read_returns_none_for_empty_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        fs::File::create(&path).unwrap();
        let text = read(path).unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn reads_returns_the_content_of_file_as_string() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        fs::write(&path, "hello world").unwrap();
        let text = read(path).unwrap().unwrap();
        assert_eq!(text.trim_end(), "hello world", "wrong text");
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello logbook\n", "wrong text");
    }

    #[test]
    fn append_appends_line_to_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.txt");
        fs::write(&path, "hello\n").unwrap();
        append(&path, "logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello\nlogbook\n", "wrong text");
    }
}
