use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/1a.txt";

    let max = a(&puzzle_data);
    println!("Max calories carried is: {}", max);

    let b_result = b(&puzzle_data);
    println!("Total carried by top three is: {}", b_result);
}

fn build_elves_vec(path: &str) -> Vec<Vec<u32>> {
    let lines = read_lines(path).unwrap();

    let mut elves: Vec<Vec<u32>> = Vec::new();

    let mut elf: Vec<u32> = Vec::new();
    for line in lines {
        if line.trim().eq("") {
            elves.push(elf.clone());
            elf = Vec::new();
        } else {
            let calories: u32 = line.parse().unwrap();
            elf.push(calories);
        }
    }
    // push last elf since no blank line after
    elves.push(elf);

    elves
}

fn a(path: &str) -> u32 {
    let elves = build_elves_vec(path);

    let mut max = 0;
    for elf in elves {
        let total = elf.iter().sum();
        if total > max {
            max = total;
        }
    }

    max
}

fn b(path: &str) -> u32 {
    let elves = build_elves_vec(path);

    let mut elves_totaled: Vec<u32> = Vec::new();
    for elf in elves {
        let total = elf.iter().sum();
        elves_totaled.push(total);
    }

    elves_totaled.sort();
    elves_totaled.reverse();

    elves_totaled[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day1.txt";
        let result = a(&test_path);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day1.txt";
        let result = b(&test_path);
        assert_eq!(result, 45000);
    }
}
