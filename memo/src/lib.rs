use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Result};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Status {
    Pending,
    Done,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            memos.inner = serde_json::from_reader(file)?;
        }
        Ok(memos)
    }

    pub fn sync(&self) -> Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = Memos::open("tests/data/memos.txt").unwrap();
        assert_eq!(
            memos.inner,
            vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Done,
                }
            ],
            "wrong data"
        );
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = Memos::open("nofile.txt").unwrap();
        assert!(memos.inner.is_empty(), "vec not empty");
    }

    #[test]
    fn sync_writes_vec_to_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Done,
                },
            ],
        };
        memos.sync().unwrap();
        let memos2 = Memos::open(&path).unwrap();
        assert_eq!(memos2.inner, memos.inner, "wrong data")
    }
}
