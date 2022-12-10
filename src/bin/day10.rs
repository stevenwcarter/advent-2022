use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/10.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    b(puzzle_data);
}

fn a(path: &str) -> i32 {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut signals: Vec<i32> = Vec::new();
    read_lines(path).unwrap().iter().for_each(|line| {
        signals.push(x);
        if line.starts_with("noop") {
            cycle += 1;
        } else {
            let amount_string = line.strip_prefix("addx ").unwrap();
            let amount = amount_string.parse::<i32>().unwrap();
            cycle += 1;
            signals.push(x);
            x += amount;
        }
    });

    draw_line(&signals[0..39]);
    draw_line(&signals[40..79]);
    draw_line(&signals[80..119]);
    draw_line(&signals[120..159]);
    draw_line(&signals[160..199]);
    draw_line(&signals[200..239]);

    20 * signals[19]
        + 60 * signals[59]
        + 100 * signals[99]
        + 140 * signals[139]
        + 180 * signals[179]
        + 220 * signals[219]
}

fn draw_line(array: &[i32]) {
    let mut pos: i32 = 0;
    array.iter().for_each(|val| {
        if (val - pos).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
        pos += 1;
    });
    println!();
}

fn b(path: &str) {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut signals: Vec<i32> = Vec::new();
    read_lines(path).unwrap().iter().for_each(|line| {
        signals.push(x);
        if line.starts_with("noop") {
            cycle += 1;
        } else {
            let amount_string = line.strip_prefix("addx ").unwrap();
            let amount = amount_string.parse::<i32>().unwrap();
            cycle += 1;
            signals.push(x);
            x += amount;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day10-2.txt";
        let result = a(test_path);
        assert_eq!(result, 13140);
    }

    // #[test]
    // fn test_b() {
    //     let test_path = "test-resources/day10.txt";
    //     let result = b(test_path);
    //     assert_eq!(result, 70);
    // }

    #[test]
    fn test_b2() {
        let test_path = "test-resources/day10-2.txt";
        b(test_path);
    }
}
