use advent::read_lines;

fn build_stacks() -> Vec<Vec<char>> {
    let stacks: Vec<Vec<char>> = vec![
        "RPCDBG".chars().collect(),
        "HVG".chars().collect(),
        "NSQDJPM".chars().collect(),
        "PSLGDCNM".chars().collect(),
        "JBNCPFLS".chars().collect(),
        "QBDZVGTS".chars().collect(),
        "BZMHFTQ".chars().collect(),
        "CMDBF".chars().collect(),
        "FCQG".chars().collect(),
    ];

    stacks
}

fn main() {
    let puzzle_data = "puzzles/5.txt";

    let mut stacks = build_stacks();

    let answer_a = a(puzzle_data, &mut stacks);
    println!("Order of top crates (1-by-1): {}", answer_a);

    let mut stacks = build_stacks();

    let b_result = b(puzzle_data, &mut stacks);
    println!("Order of top crates (total-move): {}", b_result);
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
    let quantity: usize = parts[1].parse::<usize>().unwrap();
    let source: usize = parts[3].parse::<usize>().unwrap() - 1;
    let dest: usize = parts[5].parse::<usize>().unwrap() - 1;

    (quantity, source, dest)
}

fn parse_result(stacks: &mut [Vec<char>]) -> String {
    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    result
}

fn a(path: &str, stacks: &mut [Vec<char>]) -> String {
    read_lines(path)
        .unwrap()
        .iter()
        .filter(|l| l.contains("move"))
        .map(|line| parse_line(line))
        .for_each(|(quantity, source, dest)| {
            let mut crates_to_move = stacks[source].split_off(stacks[source].len() - quantity);
            crates_to_move.reverse();
            stacks[dest].append(&mut crates_to_move);
        });

    parse_result(stacks)
}

fn b(path: &str, stacks: &mut [Vec<char>]) -> String {
    read_lines(path)
        .unwrap()
        .iter()
        .filter(|l| l.contains("move"))
        .map(|line| parse_line(line))
        .for_each(|(quantity, source, dest)| {
            let mut crates_to_move = stacks[source].split_off(stacks[source].len() - quantity);
            stacks[dest].append(&mut crates_to_move);
        });

    parse_result(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stack1: Vec<char> = "ZN".chars().collect();
        let stack2: Vec<char> = "MCD".chars().collect();
        let stack3: Vec<char> = "P".chars().collect();

        stacks.push(stack1);
        stacks.push(stack2);
        stacks.push(stack3);
        let test_path = "test-resources/day5.txt";
        let result = a(test_path, &mut stacks);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_a_final() {
        let mut stacks = build_stacks();
        let path = "puzzles/5.txt";
        let result = a(path, &mut stacks);
        assert_eq!(result, "TLNGFGMFN");
    }

    #[test]
    fn test_b() {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stack1: Vec<char> = "ZN".chars().collect();
        let stack2: Vec<char> = "MCD".chars().collect();
        let stack3: Vec<char> = "P".chars().collect();

        stacks.push(stack1);
        stacks.push(stack2);
        stacks.push(stack3);
        let test_path = "test-resources/day5.txt";
        let result = b(test_path, &mut stacks);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn test_b_final() {
        let mut stacks = build_stacks();
        let path = "puzzles/5.txt";
        let result = b(path, &mut stacks);
        assert_eq!(result, "FGLQJCMBD");
    }
}
