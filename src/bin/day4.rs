use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/4.txt";

    let elves = parse_elves(puzzle_data);

    let answer_a = a(&elves);
    println!("Total completely overlapping is: {}", answer_a);

    let b_result = b(&elves);
    println!("Total somewhat overlapping is: {}", b_result);
}

pub struct Elf {
    lower: u32,
    upper: u32,
}

impl Elf {
    pub fn is_contained(self: &Elf, elf2: &Elf) -> bool {
        (elf2.lower >= self.lower && elf2.upper <= self.upper)
            || (self.lower >= elf2.lower && self.upper <= elf2.upper)
    }
    pub fn overlaps(self: &Elf, elf2: &Elf) -> bool {
        !(elf2.lower > self.upper || elf2.upper < self.lower)
    }
}

fn parse_elves(path: &str) -> Vec<(Elf, Elf)> {
    let lines = read_lines(path).unwrap();

    let elves_vec: Vec<(Elf, Elf)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            let elf1_parts: Vec<u32> = parts[0]
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let elf2_parts: Vec<u32> = parts[1]
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            (
                Elf {
                    lower: elf1_parts[0],
                    upper: elf1_parts[1],
                },
                Elf {
                    lower: elf2_parts[0],
                    upper: elf2_parts[1],
                },
            )
        })
        .collect();

    elves_vec
}

fn a(elves: &[(Elf, Elf)]) -> u32 {
    elves
        .iter()
        .filter(|(elf1, elf2)| elf1.is_contained(elf2))
        .count() as u32
}

fn b(elves: &[(Elf, Elf)]) -> u32 {
    elves
        .iter()
        .filter(|(elf1, elf2)| elf1.overlaps(elf2))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day4.txt";
        let elves = parse_elves(test_path);
        let result = a(&elves);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day4.txt";
        let elves = parse_elves(test_path);
        let result = b(&elves);
        assert_eq!(result, 4);
    }
}
