use std::collections::HashSet;

use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/9.txt";

    let (answer_a, answer_b) = a(puzzle_data);

    println!("A Score: {}", answer_a);
    println!("B Score: {}", answer_b);
}

fn distance_too_great_and_adjust(i: usize, x: &mut [i32], y: &mut [i32]) {
    if x[i] - x[i + 1] > 1 {
        if y[i] - y[i + 1] > 1 {
            y[i + 1] = y[i] - 1;
        } else if y[i] - y[i + 1] < -1 {
            y[i + 1] = y[i] + 1;
        } else {
            y[i + 1] = y[i];
        }
        x[i + 1] = x[i] - 1;
    } else if x[i] - x[i + 1] < -1 {
        if y[i] - y[i + 1] > 1 {
            y[i + 1] = y[i] - 1;
        } else if y[i] - y[i + 1] < -1 {
            y[i + 1] = y[i] + 1;
        } else {
            y[i + 1] = y[i];
        }
        x[i + 1] = x[i] + 1;
    } else if y[i] - y[i + 1] > 1 {
        if x[i] - x[i + 1] > 1 {
            x[i + 1] = x[i] - 1;
        } else if x[i] - x[i + 1] < -1 {
            x[i + 1] = x[i] + 1;
        } else {
            x[i + 1] = x[i];
        }
        x[i + 1] = x[i];
        y[i + 1] = y[i] - 1;
    } else if y[i] - y[i + 1] < -1 {
        if x[i] - x[i + 1] > 1 {
            x[i + 1] = x[i] - 1;
        } else if x[i] - x[i + 1] < -1 {
            x[i + 1] = x[i] + 1;
        } else {
            x[i + 1] = x[i];
        }
        x[i + 1] = x[i];
        y[i + 1] = y[i] + 1;
    }
}

fn a(path: &str) -> (usize, usize) {
    // starting positions
    let mut tx_vec: Vec<i32> = vec![0; 10];
    let mut ty_vec: Vec<i32> = vec![0; 10];

    // keep track of where two of the knots "visit"
    let mut visited_a: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_b: HashSet<(i32, i32)> = HashSet::new();

    read_lines(path).unwrap().iter().for_each(|line| {
        let mut chars = line.chars();
        let direction: char = chars.next().unwrap();
        chars.next(); // skip the space
        let remaining: String = chars.collect();
        let distance: i32 = remaining.parse::<i32>().unwrap();

        for _ in 0..distance {
            if direction == 'U' {
                ty_vec[0] += 1;
            } else if direction == 'D' {
                ty_vec[0] -= 1;
            } else if direction == 'L' {
                tx_vec[0] -= 1;
            } else {
                tx_vec[0] += 1;
            }

            for i in 0..9 {
                distance_too_great_and_adjust(i, &mut tx_vec, &mut ty_vec);
            }

            visited_a.insert((tx_vec[1], ty_vec[1]));
            visited_b.insert((tx_vec[9], ty_vec[9]));
        }
    });

    (visited_a.len(), visited_b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day9.txt";
        let (result, _) = a(test_path);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day9.txt";
        let (_, result) = a(test_path);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_b_longer() {
        let test_path = "test-resources/day9-2.txt";
        let (_, result) = a(test_path);
        assert_eq!(result, 36);
    }
    #[test]
    fn test_b_real() {
        let test_path = "puzzles/9.txt";
        let (_, result) = a(test_path);
        println!("Real b result: {}", result);
        assert!(result > 2529);
    }
}
