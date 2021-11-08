use anyhow::Result;
use std::io::BufRead;
use std::io::Write;

pub fn find_matches<R, W>(mut reader: R, pattern: &str, mut writer: W) -> Result<usize>
where
    R: BufRead,
    W: Write,
{
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)?;

        if len == 0 {
            return Ok(0);
        }

        if line.contains(&pattern) {
            writeln!(writer, "{}", line.trim())?;
        }
        line.clear();
    }
}

#[test]
fn find_a_match() {
    let reader = String::from("lorem ipsum\ndolor sit amet");
    let mut result = Vec::new();
    let pattern = String::from("lorem");
    find_matches(reader.as_bytes(), &pattern, &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
