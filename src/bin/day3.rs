use advent::read_lines;
use array_tool::vec::Intersect;

fn main() {
    let puzzle_data = "puzzles/3.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

// Use the ascii codes for score, just adjust to make them fit the scale
fn letter_score(letter: char) -> u32 {
    let code = letter as u32;
    if code > 96 {
        code - 96
    } else {
        code - 64 + 26
    }
}

fn split_line(line: &str) -> (&str, &str) {
    let len = line.len() / 2;

    line.split_at(len)
}

fn a(path: &str) -> u32 {
    read_lines(path)
        .unwrap()
        .iter()
        .map(|line| {
            let (left, right) = split_line(line);

            let left: Vec<char> = left.chars().collect();
            let right: Vec<char> = right.chars().collect();

            let common: Vec<char> = left.intersect(right);
            letter_score(common[0])
        })
        .sum()
}

fn b(path: &str) -> u32 {
    let lines = read_lines(path).unwrap();
    let len = lines.len();

    let mut score = 0;
    let mut index = 0;
    while index < len - 2 {
        let aa: Vec<char> = lines[index].chars().collect();
        let bb: Vec<char> = lines[index + 1].chars().collect();
        let cc: Vec<char> = lines[index + 2].chars().collect();

        let common: Vec<char> = aa.intersect(bb);
        let common: Vec<char> = common.intersect(cc);

        score += letter_score(common[0]);

        index += 3;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_code() {
        let code_a = letter_score('a');
        assert_eq!(code_a, 1);
        let code_a_large = letter_score('A');
        assert_eq!(code_a_large, 27);
    }

    #[test]
    fn test_a() {
        let test_path = "test-resources/day3.txt";
        let result = a(test_path);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day3.txt";
        let result = b(test_path);
        assert_eq!(result, 70);
    }
}
