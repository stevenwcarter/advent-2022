use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/5.txt";

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let stack1 = vec!['R', 'P', 'C', 'D', 'B', 'G'];
    let stack2 = vec!['H', 'V', 'G'];
    let stack3: Vec<char> = "NSQDJPM".chars().collect();
    let stack4: Vec<char> = "PSLGDCNM".chars().collect();
    let stack5: Vec<char> = "JBNCPFLS".chars().collect();
    let stack6: Vec<char> = "QBDZVGTS".chars().collect();
    let stack7: Vec<char> = "BZMHFTQ".chars().collect();
    let stack8: Vec<char> = "CMDBF".chars().collect();
    let stack9: Vec<char> = "FCQG".chars().collect();

    stacks.push(stack1);
    stacks.push(stack2);
    stacks.push(stack3);
    stacks.push(stack4);
    stacks.push(stack5);
    stacks.push(stack6);
    stacks.push(stack7);
    stacks.push(stack8);
    stacks.push(stack9);

    let answer_a = a(puzzle_data, &mut stacks);
    println!("Total completely overlapping is: {}", answer_a);

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let stack1 = vec!['R', 'P', 'C', 'D', 'B', 'G'];
    let stack2 = vec!['H', 'V', 'G'];
    let stack3: Vec<char> = "NSQDJPM".chars().collect();
    let stack4: Vec<char> = "PSLGDCNM".chars().collect();
    let stack5: Vec<char> = "JBNCPFLS".chars().collect();
    let stack6: Vec<char> = "QBDZVGTS".chars().collect();
    let stack7: Vec<char> = "BZMHFTQ".chars().collect();
    let stack8: Vec<char> = "CMDBF".chars().collect();
    let stack9: Vec<char> = "FCQG".chars().collect();

    stacks.push(stack1);
    stacks.push(stack2);
    stacks.push(stack3);
    stacks.push(stack4);
    stacks.push(stack5);
    stacks.push(stack6);
    stacks.push(stack7);
    stacks.push(stack8);
    stacks.push(stack9);

    let b_result = b(puzzle_data, &mut stacks);
    println!("Total somewhat overlapping is: {}", b_result);
}

fn a(path: &str, stacks: &mut [Vec<char>]) -> String {
    let lines = read_lines(path).unwrap();

    for line in lines {
        if line.contains("move") {
            let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
            let quantity: u32 = parts[1].parse::<u32>().unwrap();
            let source: u32 = parts[3].parse::<u32>().unwrap() - 1;
            let dest: u32 = parts[5].parse::<u32>().unwrap() - 1;

            for _ in 0..quantity {
                let char = stacks[source as usize].pop().unwrap();
                stacks[dest as usize].push(char);
            }
        }
    }

    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    result
}

fn b(path: &str, stacks: &mut [Vec<char>]) -> String {
    let lines = read_lines(path).unwrap();

    for line in lines {
        if line.contains("move") {
            let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
            let quantity: u32 = parts[1].parse::<u32>().unwrap();
            let source: u32 = parts[3].parse::<u32>().unwrap() - 1;
            let dest: u32 = parts[5].parse::<u32>().unwrap() - 1;

            let mut crates: Vec<char> = Vec::new();
            for _ in 0..quantity {
                crates.push(stacks[source as usize].pop().unwrap());
            }
            crates.reverse();
            for _ in 0..quantity {
                stacks[dest as usize].append(&mut crates);
            }
        }
    }

    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    result
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
}
