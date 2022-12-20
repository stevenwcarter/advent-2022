use advent::read_lines;
use core::fmt;
use moveslice::Moveslice;
use rayon::prelude::*;
use uuid::Uuid;

fn main() {
    let puzzle_data = "puzzles/20.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Signal {
    id: String,
    val: i64,
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signal").field("val", &self.val).finish()
    }
}

impl Signal {
    pub fn new(val: i64) -> Signal {
        Signal {
            id: Uuid::new_v4().to_string(),
            val,
        }
    }
}

fn get_index(signals: &[Signal], pos: usize) -> i64 {
    let len = signals.len();
    let start_index = signals.iter().position(|s| s.val.eq(&0)).unwrap();

    let new_pos = (start_index as i64 + pos as i64) % len as i64;

    signals[new_pos as usize].val
}

fn a(path: &str) -> i64 {
    let mut signals: Vec<Signal> = read_lines(path)
        .unwrap()
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .map(Signal::new)
        .collect();

    let orig_signals = signals.clone();
    let len = orig_signals.len();

    orig_signals.iter().for_each(|os| {
        let index = signals.iter().position(|s| s.eq(os)).unwrap();
        let mut new_pos = index as i64 + os.val;
        while new_pos <= 0 {
            new_pos += len as i64 - 1;
        }
        while new_pos >= len as i64 {
            new_pos %= len as i64 - 1;
        }
        signals.moveslice(index..index + 1, new_pos as usize);
    });

    get_index(&signals, 1000) + get_index(&signals, 2000) + get_index(&signals, 3000)
}

fn b(path: &str) -> i64 {
    const KEY: i64 = 811589153;
    let mut signals: Vec<Signal> = read_lines(path)
        .unwrap()
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|val| val * KEY)
        .map(Signal::new)
        .collect();

    let orig_signals = signals.clone();
    let len = orig_signals.len();

    (0..10).into_iter().for_each(|_| {
        orig_signals.iter().for_each(|os| {
            let index = signals.par_iter().position_first(|s| s.eq(os)).unwrap();
            let mut new_pos = index as i64 + os.val;
            while new_pos <= 0 {
                new_pos += KEY * 10000 * (len as i64 - 1);
            }
            new_pos %= len as i64 - 1;
            signals.moveslice(index..index + 1, new_pos as usize);
        });
    });

    get_index(&signals, 1000) + get_index(&signals, 2000) + get_index(&signals, 3000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day20.txt";
        let result = a(test_path);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day20.txt";
        let result = b(test_path);
        assert_eq!(result, 1623178306);
    }

    #[test]
    fn test_20b_actual() {
        let test_path = "puzzles/20.txt";
        let result = b(test_path);
        assert_eq!(result, 7865110481723);
    }
}
