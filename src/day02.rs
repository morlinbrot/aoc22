use std::cmp::Ordering;

// use crate::util;

// A - X - Rock - 1
// B - Y - Paper - 2
// C - Z - Scissors - 3

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/day02_input.txt").unwrap();

    let mut sum = 0;
    for line in str.lines() {
        let split: Vec<usize> = line
            .split_whitespace()
            .map(|elm| match elm {
                "A" | "X" => 1,
                "B" | "Y" => 2,
                "C" | "Z" => 3,
                _ => unreachable!(),
            })
            .collect();

        let res = match split[..] {
            [you, me] => match me.cmp(&you) {
                Ordering::Greater => me + 6,
                Ordering::Equal => me + 3,
                Ordering::Less => me + 0,
            },
            _ => unreachable!(),
        };

        sum += res;
    }

    sum
}
