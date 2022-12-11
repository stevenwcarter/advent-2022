use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/11.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

fn a(path: &str) -> u32 {
    read_lines(path).unwrap().iter().for_each(|_line| {
        //
    });

    0
}

fn b(_path: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day11.txt";
        let result = a(test_path);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day11.txt";
        let result = b(test_path);
        assert_eq!(result, 70);
    }
}
