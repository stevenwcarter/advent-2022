use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/2a.txt";

    let answer_a = a(&puzzle_data);
    println!("Total score: {}", answer_a);

    let b_result = b(&puzzle_data);
    println!("Total carried by top three is: {}", b_result);
}

enum RPSInput {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RPSInput {
    pub fn lose_response(&self) -> RPSOutput {
        match self {
            RPSInput::ROCK => RPSOutput::SCISSORS,
            RPSInput::SCISSORS => RPSOutput::PAPER,
            RPSInput::PAPER => RPSOutput::ROCK,
        }
    }
    pub fn draw_response(&self) -> RPSOutput {
        match self {
            RPSInput::ROCK => RPSOutput::ROCK,
            RPSInput::SCISSORS => RPSOutput::SCISSORS,
            RPSInput::PAPER => RPSOutput::PAPER,
        }
    }
    pub fn win_response(&self) -> RPSOutput {
        match self {
            RPSInput::ROCK => RPSOutput::PAPER,
            RPSInput::SCISSORS => RPSOutput::ROCK,
            RPSInput::PAPER => RPSOutput::SCISSORS,
        }
    }
}

enum RPSOutput {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RPSOutput {
    pub fn score(&self) -> u32 {
        match self {
            RPSOutput::ROCK => 1,
            RPSOutput::PAPER => 2,
            RPSOutput::SCISSORS => 3,
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
            RPSInput::ROCK => match self.output {
                RPSOutput::ROCK => self.output.score() + 3,
                RPSOutput::PAPER => self.output.score() + 6,
                RPSOutput::SCISSORS => self.output.score() + 0,
            },
            RPSInput::PAPER => match self.output {
                RPSOutput::ROCK => self.output.score() + 0,
                RPSOutput::PAPER => self.output.score() + 3,
                RPSOutput::SCISSORS => self.output.score() + 6,
            },
            RPSInput::SCISSORS => match self.output {
                RPSOutput::ROCK => self.output.score() + 6,
                RPSOutput::PAPER => self.output.score() + 0,
                RPSOutput::SCISSORS => self.output.score() + 3,
            },
        }
    }
}

fn a(path: &str) -> u32 {
    let lines = read_lines(path).unwrap();

    let mut rounds: Vec<Strategy> = Vec::new();
    for line in lines {
        let splits = line.trim().split(" ");
        let splits: Vec<&str> = splits.into_iter().collect();
        let input: RPSInput = match splits[0] {
            "A" => RPSInput::ROCK,
            "B" => RPSInput::PAPER,
            "C" => RPSInput::SCISSORS,
            _ => panic!("Unknown input"),
        };
        let output: RPSOutput = match splits[1] {
            "X" => RPSOutput::ROCK,
            "Y" => RPSOutput::PAPER,
            "Z" => RPSOutput::SCISSORS,
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
        let splits = line.trim().split(" ");
        let splits: Vec<&str> = splits.into_iter().collect();
        let input: RPSInput = match splits[0] {
            "A" => RPSInput::ROCK,
            "B" => RPSInput::PAPER,
            "C" => RPSInput::SCISSORS,
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
        let result = a(&test_path);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day2.txt";
        let result = b(&test_path);
        assert_eq!(result, 12);
    }
}
