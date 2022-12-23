use std::collections::HashMap;

use advent::read_lines;

fn main() {
    let puzzle_data = "puzzles/23.txt";

    let answer_a = a(puzzle_data);
    println!("A Score: {}", answer_a);

    let b_result = b(puzzle_data);
    println!("B Score: {}", b_result);
}

fn north_clear(elves: &[(i32, i32)], r: &i32, c: &i32) -> bool {
    !(elves.contains(&(*r - 1, *c - 1))
        || elves.contains(&(*r - 1, *c))
        || elves.contains(&(*r - 1, *c + 1)))
}
fn south_clear(elves: &[(i32, i32)], r: &i32, c: &i32) -> bool {
    !(elves.contains(&(r + 1, c - 1))
        || elves.contains(&(r + 1, *c))
        || elves.contains(&(r + 1, c + 1)))
}
fn west_clear(elves: &[(i32, i32)], r: &i32, c: &i32) -> bool {
    !(elves.contains(&(r + 1, c - 1))
        || elves.contains(&(*r, c - 1))
        || elves.contains(&(r - 1, c - 1)))
}
fn east_clear(elves: &[(i32, i32)], r: &i32, c: &i32) -> bool {
    !(elves.contains(&(r + 1, c + 1))
        || elves.contains(&(*r, c + 1))
        || elves.contains(&(r - 1, c + 1)))
}

fn empty_surrounds(elves: &[(i32, i32)], r: &i32, c: &i32) -> bool {
    !(elves.contains(&(r - 1, c - 1))
        || elves.contains(&(r - 1, *c))
        || elves.contains(&(r - 1, c + 1))
        || elves.contains(&(*r, c - 1))
        || elves.contains(&(*r, c + 1))
        || elves.contains(&(r + 1, c - 1))
        || elves.contains(&(r + 1, *c))
        || elves.contains(&(r + 1, c + 1)))
}

fn calc_round(round_num: usize, elves: &mut Vec<(i32, i32)>) -> usize {
    let mut new_elves: HashMap<(i32, i32), u32> = HashMap::new();
    elves.iter().for_each(|(r, c)| {
        if !empty_surrounds(elves, r, c) {
            if round_num % 4 == 0 {
                if north_clear(elves, r, c) {
                    new_elves
                        .entry((r - 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if south_clear(elves, r, c) {
                    new_elves
                        .entry((r + 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if west_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c - 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if east_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c + 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            } else if round_num % 4 == 1 {
                if south_clear(elves, r, c) {
                    new_elves
                        .entry((r + 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if west_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c - 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if east_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c + 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if north_clear(elves, r, c) {
                    new_elves
                        .entry((r - 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            } else if round_num % 4 == 2 {
                if west_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c - 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if east_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c + 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if north_clear(elves, r, c) {
                    new_elves
                        .entry((r - 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if south_clear(elves, r, c) {
                    new_elves
                        .entry((r + 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            } else if round_num % 4 == 3 {
                if east_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c + 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if north_clear(elves, r, c) {
                    new_elves
                        .entry((r - 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if south_clear(elves, r, c) {
                    new_elves
                        .entry((r + 1, *c))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                } else if west_clear(elves, r, c) {
                    new_elves
                        .entry((*r, c - 1))
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
        }
    });

    if new_elves.len() == 0 {
        return 0 as usize;
    }
    // new_elves.iter().for_each(|((r, c), v)| {
    //     println!("({},{}): {}", r, c, v);
    // });
    let elf_check_clone = elves.clone();
    for (r, c) in elves.clone() {
        if !empty_surrounds(&elf_check_clone, &r, &c) {
            if round_num % 4 == 0 {
                if north_clear(&elf_check_clone, &r, &c) {
                    // println!("Checking {},{}", r - 1, c);
                    if *new_elves.get(&(r - 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r - 1, c));
                    }
                } else if south_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r + 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r + 1, c));
                    }
                } else if west_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c - 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c - 1));
                    }
                } else if east_clear(&elf_check_clone, &r, &c)
                    && *new_elves.get(&(r, c + 1)).unwrap() == 1
                {
                    elves.remove(
                        elves
                            .iter()
                            .position(|(nr, nc)| *nr == r && *nc == c)
                            .expect("Could not find elf"),
                    );
                    elves.push((r, c + 1));
                }
            } else if round_num % 4 == 1 {
                if south_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r + 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r + 1, c));
                    }
                } else if west_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c - 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c - 1));
                    }
                } else if east_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c + 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c + 1));
                    }
                } else if north_clear(&elf_check_clone, &r, &c)
                    && *new_elves.get(&(r - 1, c)).unwrap() == 1
                {
                    elves.remove(
                        elves
                            .iter()
                            .position(|(nr, nc)| *nr == r && *nc == c)
                            .expect("Could not find elf"),
                    );
                    elves.push((r - 1, c));
                }
            } else if round_num % 4 == 2 {
                if west_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c - 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c - 1));
                    }
                } else if east_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c + 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c + 1));
                    }
                } else if north_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r - 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r - 1, c));
                    }
                } else if south_clear(&elf_check_clone, &r, &c)
                    && *new_elves.get(&(r + 1, c)).unwrap() == 1
                {
                    elves.remove(
                        elves
                            .iter()
                            .position(|(nr, nc)| *nr == r && *nc == c)
                            .expect("Could not find elf"),
                    );
                    elves.push((r + 1, c));
                }
            } else if round_num % 4 == 3 {
                if east_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r, c + 1)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r, c + 1));
                    }
                } else if north_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r - 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r - 1, c));
                    }
                } else if south_clear(&elf_check_clone, &r, &c) {
                    if *new_elves.get(&(r + 1, c)).unwrap() == 1 {
                        elves.remove(
                            elves
                                .iter()
                                .position(|(nr, nc)| *nr == r && *nc == c)
                                .expect("Could not find elf"),
                        );
                        elves.push((r + 1, c));
                    }
                } else if west_clear(&elf_check_clone, &r, &c)
                    && *new_elves.get(&(r, c - 1)).unwrap() == 1
                {
                    elves.remove(
                        elves
                            .iter()
                            .position(|(nr, nc)| *nr == r && *nc == c)
                            .expect("Could not find elf"),
                    );
                    elves.push((r, c - 1));
                }
            }
        }
    }

    return new_elves.len();
}

fn calc_bounds_size(elves: &[(i32, i32)]) -> u32 {
    let min_x = elves.iter().map(|(_, c)| c).min().unwrap();
    let max_x = elves.iter().map(|(_, c)| c).max().unwrap();
    let min_y = elves.iter().map(|(r, _)| r).min().unwrap();
    let max_y = elves.iter().map(|(r, _)| r).max().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // println!(
    //     "min_x: {}\nmax_x: {}\nmin_y: {}\nmax_y: {}\nwidth: {}\nheight: {}",
    //     min_x, max_x, min_y, max_y, width, height
    // );

    let elves_count = elves.len();

    // println!("Elves count: {}", elves_count);

    (width * height) as u32 - elves_count as u32
}

fn a(path: &str) -> u32 {
    let mut elves: Vec<(i32, i32)> = Vec::new();
    let mut row = 0;
    read_lines(path).unwrap().iter().for_each(|line| {
        let mut col = 0;
        line.chars().into_iter().for_each(|c| {
            if c == '#' {
                elves.push((row, col));
            }

            col += 1;
        });
        row += 1;
    });

    // assert_eq!(calc_bounds_size(&elves), 27);

    for round_num in 0..10 {
        calc_round(round_num, &mut elves);
    }

    calc_bounds_size(&elves)
}
fn b(path: &str) -> u32 {
    let mut elves: Vec<(i32, i32)> = Vec::new();
    let mut row = 0;
    read_lines(path).unwrap().iter().for_each(|line| {
        let mut col = 0;
        line.chars().into_iter().for_each(|c| {
            if c == '#' {
                elves.push((row, col));
            }

            col += 1;
        });
        row += 1;
    });

    // assert_eq!(calc_bounds_size(&elves), 27);

    let mut round_num: u32 = 0;
    while calc_round(round_num as usize, &mut elves) != 0 {
        round_num += 1;
    }

    round_num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_path = "test-resources/day23.txt";
        let result = a(test_path);
        assert_eq!(result, 110);
    }

    #[test]
    fn test_b() {
        let test_path = "test-resources/day23.txt";
        let result = b(test_path);
        assert_eq!(result, 20);
    }
}
