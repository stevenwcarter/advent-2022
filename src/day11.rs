pub struct Monkey {
    pub items: Vec<u64>,
    pub inspection_count: u64,
    operation: Box<dyn Fn(&u64) -> u64>,
    pub test: u64,
    if_true_dest: usize,
    if_false_dest: usize,
}
pub fn build_test_monkeys() -> Vec<Monkey> {
    let monkeys = vec![
        Monkey {
            items: vec![79, 98],
            inspection_count: 0,
            operation: Box::new(|item| item * 19),
            test: 23,
            if_true_dest: 2,
            if_false_dest: 3,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            inspection_count: 0,
            operation: Box::new(|item| item + 6),
            test: 19,
            if_true_dest: 2,
            if_false_dest: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            inspection_count: 0,
            operation: Box::new(|item| item * item),
            test: 13,
            if_true_dest: 1,
            if_false_dest: 3,
        },
        Monkey {
            items: vec![74],
            inspection_count: 0,
            operation: Box::new(|item| item + 3),
            test: 17,
            if_true_dest: 0,
            if_false_dest: 1,
        },
    ];
    monkeys
}
pub fn build_monkeys() -> Vec<Monkey> {
    let monkeys = vec![
        Monkey {
            items: vec![74, 73, 57, 77, 74],
            inspection_count: 0,
            operation: Box::new(|item| item * 11),
            test: 19,
            if_true_dest: 6,
            if_false_dest: 7,
        },
        Monkey {
            items: vec![99, 77, 79],
            inspection_count: 0,
            operation: Box::new(|item| item + 8),
            test: 2,
            if_true_dest: 6,
            if_false_dest: 0,
        },
        Monkey {
            items: vec![64, 67, 50, 96, 89, 82, 82],
            inspection_count: 0,
            operation: Box::new(|item| item + 1),
            test: 3,
            if_true_dest: 5,
            if_false_dest: 3,
        },
        Monkey {
            items: vec![88],
            inspection_count: 0,
            operation: Box::new(|item| item * 7),
            test: 17,
            if_true_dest: 5,
            if_false_dest: 4,
        },
        Monkey {
            items: vec![80, 66, 98, 83, 70, 63, 57, 66],
            inspection_count: 0,
            operation: Box::new(|item| item + 4),
            test: 13,
            if_true_dest: 0,
            if_false_dest: 1,
        },
        Monkey {
            items: vec![81, 93, 90, 61, 62, 64],
            inspection_count: 0,
            operation: Box::new(|item| item + 7),
            test: 7,
            if_true_dest: 1,
            if_false_dest: 4,
        },
        Monkey {
            items: vec![69, 97, 88, 93],
            inspection_count: 0,
            operation: Box::new(|item| item * item),
            test: 5,
            if_true_dest: 7,
            if_false_dest: 2,
        },
        Monkey {
            items: vec![59, 80],
            inspection_count: 0,
            operation: Box::new(|item| item + 6),
            test: 11,
            if_true_dest: 2,
            if_false_dest: 3,
        },
    ];
    monkeys
}

impl Monkey {
    pub fn take_turn(&mut self) -> Vec<(usize, u64)> {
        let items = self.items.clone();
        self.items = Vec::new();
        let mut return_vec: Vec<(usize, u64)> = Vec::new();
        items.iter().for_each(|item| {
            self.inspection_count += 1;
            let mut item = (self.operation)(item);
            item = (item as f64 / 3.0).floor() as u64;
            let dest = if item % self.test == 0 {
                self.if_true_dest
            } else {
                self.if_false_dest
            };

            return_vec.push((dest, item));
        });

        return_vec
    }

    pub fn take_turn_b(&mut self, modulo: u64) -> Vec<(usize, u64)> {
        let items = self.items.clone();
        self.items = Vec::new();
        let mut return_vec: Vec<(usize, u64)> = Vec::new();
        items.iter().for_each(|item| {
            self.inspection_count += 1;
            let mut item = (self.operation)(item);
            item %= modulo;
            let dest = if item % self.test == 0 {
                self.if_true_dest
            } else {
                self.if_false_dest
            };

            return_vec.push((dest, item));
        });

        return_vec
    }

    pub fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}
