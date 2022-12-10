use std::{collections::HashMap, iter::Peekable};

struct Dir(Vec<Dir>, usize);

fn main() {
    let (mut lines, mut sum) = (include_str!("../../puzzles/7.txt").lines().peekable(), 0);
    a2(&mut lines, &mut sum);
    println!("A: {}", sum);

    let result = a(&mut include_str!("../../puzzles/7.txt").lines().peekable());
    println!("A: {}", result);

    let base = build(&mut include_str!("../../puzzles/7.txt").lines().peekable());
    let b_result = b(&base, base.1 - 40_000_000);
    println!("B Total: {}", b_result.unwrap());
}

fn build(lines: &mut Peekable<impl Iterator<Item = &'static str>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(i) = lines.next() {
        match i {
            "$ cd .." => break,
            _ if i.starts_with("$ ls") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => dirs.push(build(lines)),
        }
    }
    size += dirs.iter().map(|d| d.1).sum::<usize>();
    Dir(dirs, size)
}

fn a2(lines: &mut Peekable<impl Iterator<Item = &'static str>>, sum: &mut usize) -> usize {
    let mut size = 0;
    while let Some(i) = lines.next() {
        match i {
            "$ cd .." => break,
            _ if i.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => size += a2(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

fn a(lines: &mut Peekable<impl Iterator<Item = &'static str>>) -> usize {
    let mut size_map: HashMap<String, usize> = HashMap::new();
    let mut directory: Vec<String> = vec!["/".to_string()];
    lines.for_each(|line| {
        println!("Line: {}", line);
        if let Some(new_dir) = line.strip_prefix("$ cd ") {
            if new_dir.eq("..") {
                directory.pop();
            } else if new_dir.eq("/") {
                directory = vec!["/".to_string()];
            } else {
                directory.push(new_dir.to_string() + "/");
            }
        } else if let Some(_dir_ls) = line.strip_prefix("dir ") {
            // ignored
        } else if let Some(_ls_command) = line.strip_prefix("$ ls") {
            // ignored
        } else {
            let mut line_iter = line.split(' ');
            let size_string = line_iter.next().unwrap();

            let directory = directory.join("");
            let size =
                size_map.get(&directory).unwrap_or(&0) + size_string.parse::<usize>().unwrap();
            size_map.insert(directory, size);
        }
    });

    // let mut total_size_map: HashMap<String, u64> = HashMap::new();
    let mut total: usize = 0;

    for (dir, size) in &size_map {
        let mut total_size: usize = *size;
        total_size += size_map
            .iter()
            .filter(|(other_dir, _)| other_dir.starts_with(dir))
            .filter(|(other_dir, _)| other_dir.len() > dir.len())
            .map(|(_, other_size)| other_size)
            .sum::<usize>();

        if total_size <= 100_000 {
            total += total_size;
        }
    }

    total
}

fn b(d: &Dir, min: usize) -> Option<usize> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), b(d, min)])
        .flatten()
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = a(&mut include_str!("../../test-resources/day7.txt")
            .lines()
            .peekable());
        assert_eq!(result, 95437);
    }
    #[test]
    fn test_a_real() {
        let result = a(&mut include_str!("../../puzzles/7.txt").lines().peekable());
        println!("Result: {}", result);
        assert!(result > 1232219);
    }
}
