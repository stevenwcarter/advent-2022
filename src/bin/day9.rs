use std::collections::HashSet;

use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/9.txt";

    let (answer_a, answer_b) = a(puzzle_data);
    println!("A Score: {}", answer_a);

    println!("B Score: {}", answer_b);
}

fn distance_too_great_and_adjust(hx: &i32, hy: &i32, tx: &mut i32, ty: &mut i32) {
    if *hx - *tx > 1 {
        if *hy - *ty > 1 {
            *ty = *hy - 1;
        } else if *hy - *ty < -1 {
            *ty = *hy + 1;
        } else {
            *ty = *hy;
        }
        *tx = *hx - 1;
    } else if *hx - *tx < -1 {
        if *hy - *ty > 1 {
            *ty = *hy - 1;
        } else if *hy - *ty < -1 {
            *ty = *hy + 1;
        } else {
            *ty = *hy;
        }
        *tx = *hx + 1;
    } else if *hy - *ty > 1 {
        if *hx - *tx > 1 {
            *tx = *hx - 1;
        } else if *hx - *tx < -1 {
            *tx = *hx + 1;
        } else {
            *tx = *hx;
        }
        *tx = *hx;
        *ty = *hy - 1;
    } else if *hy - *ty < -1 {
        if *hx - *tx > 1 {
            *tx = *hx - 1;
        } else if *hx - *tx < -1 {
            *tx = *hx + 1;
        } else {
            *tx = *hx;
        }
        *tx = *hx;
        *ty = *hy + 1;
    }
}

fn a(path: &str) -> (usize, usize) {
    let (mut hx, mut hy): (i32, i32) = (0, 0);
    let (mut tx1, mut ty1): (i32, i32) = (0, 0);
    let (mut tx2, mut ty2): (i32, i32) = (0, 0);
    let (mut tx3, mut ty3): (i32, i32) = (0, 0);
    let (mut tx4, mut ty4): (i32, i32) = (0, 0);
    let (mut tx5, mut ty5): (i32, i32) = (0, 0);
    let (mut tx6, mut ty6): (i32, i32) = (0, 0);
    let (mut tx7, mut ty7): (i32, i32) = (0, 0);
    let (mut tx8, mut ty8): (i32, i32) = (0, 0);
    let (mut tx9, mut ty9): (i32, i32) = (0, 0);

    let mut visited_a: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_b: HashSet<(i32, i32)> = HashSet::new();

    read_lines(path).unwrap().iter().for_each(|line| {
        let mut chars = line.chars();
        let direction: char = chars.next().unwrap();
        chars.next();
        let remaining: String = chars.collect();
        let distance: i32 = remaining.parse::<i32>().unwrap();

        for _ in 0..distance {
            if direction == 'U' {
                hy += 1;
            } else if direction == 'D' {
                hy -= 1;
            } else if direction == 'L' {
                hx -= 1;
            } else {
                hx += 1;
            }

            distance_too_great_and_adjust(&hx, &hy, &mut tx1, &mut ty1);
            distance_too_great_and_adjust(&tx1, &ty1, &mut tx2, &mut ty2);
            distance_too_great_and_adjust(&tx2, &ty2, &mut tx3, &mut ty3);
            distance_too_great_and_adjust(&tx3, &ty3, &mut tx4, &mut ty4);
            distance_too_great_and_adjust(&tx4, &ty4, &mut tx5, &mut ty5);
            distance_too_great_and_adjust(&tx5, &ty5, &mut tx6, &mut ty6);
            distance_too_great_and_adjust(&tx6, &ty6, &mut tx7, &mut ty7);
            distance_too_great_and_adjust(&tx7, &ty7, &mut tx8, &mut ty8);
            distance_too_great_and_adjust(&tx8, &ty8, &mut tx9, &mut ty9);
            visited_a.insert((tx1, ty1));
            visited_b.insert((tx9, ty9));
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
    fn test_internal() {
        let (x, y, mut a, mut b): (i32, i32, i32, i32) = (4, 3, 2, 5);
        distance_too_great_and_adjust(&x, &y, &mut a, &mut b);
        assert_eq!((a, b), (3, 3));
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
