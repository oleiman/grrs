#![allow(unused)]

use std::io::{BufRead, BufReader};
use std::io::{Write};

/// Search for the specified pattern in a buffered reader and write matching lines to a writer
pub fn find_matches<R: BufRead, W: Write>(
    reader: &mut R, pattern: &str, writer: &mut W) -> std::io::Result<usize> {
    let mut count = 0;
    for line in reader.lines().filter_map(|result| result.ok()) {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
            count += 1;
        }
    }
    Ok(count)
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let mut reader = BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    let count = find_matches(&mut reader, "lorem", &mut result);
    assert!(count.is_ok());
    assert_eq!(count.unwrap(), 1);
    assert_eq!(result, b"lorem ipsum\n");
}

#[ignore]
#[test]
fn buffer_full() {
    // TODO(oren): what's a writeable that could trigger an io error?
    let mut result = Vec::new();
    let mut reader = BufReader::new("foo\nbar\nbaz\nfoo".as_bytes());
    let count = find_matches(&mut reader, "foo", &mut result);
    assert!(count.is_err());
    assert_eq!(result, b"foo\n");
}
