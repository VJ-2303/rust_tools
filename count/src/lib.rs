use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Ok, Result};

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

pub fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();

    while input.read_line(&mut line)? > 0 {
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

pub fn count_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);
    count(file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor, Error, Read};

    #[test]
    fn count_counts_lines_and_words_in_input() {
        let input = Cursor::new("word1 word2\nword3");
        let count = count(input).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 3, "wrong word count");
    }

    #[test]
    fn count_returns_any_reaad_error() {
        let input = BufReader::new(ErrorReader);
        let count = count(input);
        assert!(count.is_err(), "no error returned");
    }

    struct ErrorReader;
    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(Error::new(std::io::ErrorKind::Other, "oh no"))
        }
    }
}
