use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/2a.txt";

    let answer_a = a(puzzle_data);
    println!("Total score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("Total carried by top three is: {}", b_result);
}

enum RPSInput {
    Rock,
    Paper,
    Scissors,
}

impl RPSInput {
    pub fn lose_response(&self) -> RPSOutput {
        match self {
            RPSInput::Rock => RPSOutput::Scissors,
            RPSInput::Scissors => RPSOutput::Paper,
            RPSInput::Paper => RPSOutput::Rock,
        }
    }
    pub fn draw_response(&self) -> RPSOutput {
        match self {
            RPSInput::Rock => RPSOutput::Rock,
            RPSInput::Scissors => RPSOutput::Scissors,
            RPSInput::Paper => RPSOutput::Paper,
        }
    }
    pub fn win_response(&self) -> RPSOutput {
        match self {
            RPSInput::Rock => RPSOutput::Paper,
            RPSInput::Scissors => RPSOutput::Rock,
            RPSInput::Paper => RPSOutput::Scissors,
        }
    }
}

enum RPSOutput {
    Rock,
    Paper,
    Scissors,
}

impl RPSOutput {
    pub fn score(&self) -> u32 {
        match self {
            RPSOutput::Rock => 1,
            RPSOutput::Paper => 2,
            RPSOutput::Scissors => 3,
        }
    }
}

struct Strategy {
    pub input: RPSInput,
    pub output: RPSOutput,
}

impl Strategy {
    pub fn score(&self) -> u32 {
        match self.input {
            RPSInput::Rock => match self.output {
                RPSOutput::Rock => self.output.score() + 3,
                RPSOutput::Paper => self.output.score() + 6,
                RPSOutput::Scissors => self.output.score(),
            },
            RPSInput::Paper => match self.output {
                RPSOutput::Rock => self.output.score(),
                RPSOutput::Paper => self.output.score() + 3,
                RPSOutput::Scissors => self.output.score() + 6,
            },
            RPSInput::Scissors => match self.output {
                RPSOutput::Rock => self.output.score() + 6,
                RPSOutput::Paper => self.output.score(),
                RPSOutput::Scissors => self.output.score() + 3,
            },
        }
    }
}

fn a(path: &str) -> u32 {
    let lines = read_lines(path).unwrap();

    let mut rounds: Vec<Strategy> = Vec::new();
    for line in lines {
        let splits = line.trim().split(' ');
        let splits: Vec<&str> = splits.into_iter().collect();
        let input: RPSInput = match splits[0] {
            "A" => RPSInput::Rock,
            "B" => RPSInput::Paper,
            "C" => RPSInput::Scissors,
            _ => panic!("Unknown input"),
        };
        let output: RPSOutput = match splits[1] {
            "X" => RPSOutput::Rock,
            "Y" => RPSOutput::Paper,
            "Z" => RPSOutput::Scissors,
            _ => panic!("Uknown output"),
        };

        let strategy = Strategy { input, output };

        rounds.push(strategy);
    }

    rounds.iter().map(|s| s.score()).sum()
}

fn b(path: &str) -> u32 {
    let lines = read_lines(path).unwrap();

    let mut rounds: Vec<Strategy> = Vec::new();
    for line in lines {
        let splits = line.trim().split(' ');
        let splits: Vec<&str> = splits.into_iter().collect();
        let input: RPSInput = match splits[0] {
            "A" => RPSInput::Rock,
            "B" => RPSInput::Paper,
            "C" => RPSInput::Scissors,
            _ => panic!("Unknown input"),
        };
        let output: RPSOutput = match splits[1] {
            "X" => input.lose_response(),
            "Y" => input.draw_response(),
            "Z" => input.win_response(),
            _ => panic!("Uknown output"),
        };

        let strategy = Strategy { input, output };

        rounds.push(strategy);
    }

    rounds.iter().map(|s| s.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day2.txt";
        let result = a(test_path);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day2.txt";
        let result = b(test_path);
        assert_eq!(result, 12);
    }
}
