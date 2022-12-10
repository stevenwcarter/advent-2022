use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod radio_stream;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    let reader = io::BufReader::new(file).lines();
    let lines: Vec<String> = reader.into_iter().map(|l| l.unwrap()).collect();

    Ok(lines)
}
pub fn read<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let result = read_to_string(filename).unwrap().parse();

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_lines_from_file() {
        let result = read_lines("test-resources/test.txt").unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[2], "c");
    }
}
