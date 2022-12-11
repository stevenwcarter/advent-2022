use advent::day11::build_monkeys;
use advent::day11::Monkey;

fn main() {
    let mut monkeys = build_monkeys();
    let answer_a = a(&mut monkeys);
    println!("Monkey Business A: {}", answer_a);

    let mut monkeys = build_monkeys();
    let b_result = b(&mut monkeys);
    println!("Monkey Business B: {}", b_result);
}

trait GenericMonkey {
    fn take_turn(&mut self) -> Vec<(usize, u64)>;
    fn add_item(&mut self, item: u64);
}

fn a(monkeys: &mut [Monkey]) -> u64 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let dests_and_items: Vec<(usize, u64)> = monkey.take_turn();
            dests_and_items.iter().for_each(|(dest, item)| {
                monkeys[*dest].add_item(*item);
            });
        }
    }

    let mut inspection_vec: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();
    inspection_vec.sort();
    inspection_vec.reverse();

    inspection_vec[0] * inspection_vec[1]
}

fn b(monkeys: &mut [Monkey]) -> u64 {
    let modulo = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let dests_and_items: Vec<(usize, u64)> = monkey.take_turn_b(modulo);
            dests_and_items.iter().for_each(|(dest, item)| {
                monkeys[*dest].add_item(*item);
            });
        }
    }

    let mut inspection_vec: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();
    inspection_vec.sort();
    inspection_vec.reverse();

    inspection_vec[0] * inspection_vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent::day11::build_test_monkeys;

    #[test]
    fn test_a() {
        let mut monkeys = build_test_monkeys();
        let result = a(&mut monkeys);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_b() {
        let mut monkeys = build_test_monkeys();
        let result = b(&mut monkeys);
        assert_eq!(result, 2713310158);
    }
}
