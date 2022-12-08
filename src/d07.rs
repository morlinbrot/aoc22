// Stolen from https://github.com/tsatke/aoc2022/blob/main/src/day7.rs
use std::collections::HashMap;

const INPUT: &str = include_str!("./d07_input");

pub fn parse_file_tree() -> HashMap<String, usize> {
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut cwd: Vec<&str> = Vec::new();

    for l in INPUT.lines() {
        let bytes = l.as_bytes();
        match bytes[0] {
            b'$' => match bytes[2] {
                b'c' => match bytes[5] {
                    b'.' => {
                        cwd.pop();
                    }
                    _ => {
                        cwd.push(&l[5..]);
                        sizes.insert(cwd.join(""), 0);
                    }
                },
                _ => {}
            },
            b'd' => {}
            _ => {
                let size = l.split(' ').next().unwrap().parse::<usize>().unwrap();
                for i in 0..cwd.len() {
                    let key = cwd[0..=i].join("");
                    *sizes.get_mut(&key).unwrap() += size;
                }
            }
        }
    }

    sizes
}

pub fn part_two() -> usize {
    let sizes = parse_file_tree();
    let needed = 30_000_000 - (70_000_000 - sizes.get("/").unwrap());
    *sizes.values().filter(|&&v| v >= needed).min().unwrap()
}

pub fn part_one() -> usize {
    let sizes = parse_file_tree();
    sizes.values().filter(|&&v| v <= 100_000).sum()
}
