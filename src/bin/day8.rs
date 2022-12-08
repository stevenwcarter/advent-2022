use std::error::Error;

use advent::read;

fn main() {
    let puzzle_data = "puzzles/8.txt";

    println!("A: {}", a(puzzle_data));

    println!("B: {}", b(puzzle_data));
}

fn a(path: &str) -> usize {
    let grid = read(path)
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut count = 0;

    // edges
    count += grid.len() * 2;
    count += grid[0].len() * 2;
    count -= 4;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            let r = grid.get(row).unwrap();
            let c = r.get(col).unwrap();

            if r[0..col].iter().max().unwrap() < c
                || r[col + 1..r.len()].iter().max().unwrap() < c
                || grid[0..row].iter().map(|v| v[col]).max().unwrap() < *c
                || grid[row + 1..grid.len()]
                    .iter()
                    .map(|v| v[col])
                    .max()
                    .unwrap()
                    < *c
            {
                count += 1;
            }
        }
    }

    count
}

fn b(path: &str) -> usize {
    let grid = read(path)
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut best = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            let r = grid.get(row).unwrap();
            let h = r.get(col).unwrap();

            let mut last = true;
            let scores = vec![
                r[0..col]
                    .iter()
                    .rev()
                    .take_while(|x| {
                        last = *x < h;
                        last
                    })
                    .count()
                    + usize::from(!last),
                r[col + 1..r.len()]
                    .iter()
                    .take_while(|x| {
                        last = *x < h;
                        last
                    })
                    .count()
                    + usize::from(!last),
                grid[0..row]
                    .iter()
                    .map(|v| v[col])
                    .rev()
                    .take_while(|x| {
                        last = x < h;
                        last
                    })
                    .count()
                    + usize::from(!last),
                grid[row + 1..grid.len()]
                    .iter()
                    .map(|v| v[col])
                    .take_while(|x| {
                        last = x < h;
                        last
                    })
                    .count()
                    + usize::from(!last),
            ];
            let tree_view = scores.iter().product();
            if tree_view > best {
                best = tree_view;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day8.txt";
        let result = a(test_path);
        assert_eq!(result, 21);
    }
    #[test]
    fn test_b() {
        let test_path = "test-resources/day8.txt";
        let result = b(test_path);
        assert_eq!(result, 8);
    }
}
