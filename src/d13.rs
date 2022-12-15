use std::cmp::Ordering;

// Grace à Jeremie pour une solution vraiment magnifique.
// https://github.com/jeremie-H/advent-of-code/blob/main/2022/src/day13.rs#L43-L62
const INPUT: &str = include_str!("./d13_input");

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    match (left[0], right[0]) {
        (a, b) if a == b => compare(&left[1..], &right[1..]),
        // Next two have to come first!
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => {
            let wrap = &[right[0], b']'];
            let right = &[wrap, &right[1..]].concat();
            compare(&left[1..], right)
        }
        (_, b'[') => {
            let wrap = &[left[0], b']'];
            let left = &[wrap, &left[1..]].concat();
            compare(left, &right[1..])
        }
        (_, _) => left[0].cmp(&right[0]),
    }
}

fn correct_order(pair: &str) -> bool {
    let (left, right) = pair.split_once('\n').unwrap();
    compare(left.as_bytes(), right.as_bytes()) == Ordering::Less
}

pub fn part_one() -> usize {
    INPUT
        .replace("10", "A")
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| correct_order(pair))
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_two() -> usize {
    let input = INPUT.replace("10", "A");
    let mut packets = input
        .split("\n\n")
        .flat_map(|pair| pair.split('\n'))
        .chain(["[[2]]", "[[6]]"])
        .map(|p| p.as_bytes())
        .collect::<Vec<_>>();

    packets.sort_by(|a, b| compare(a, b));

    packets
        .iter()
        .enumerate()
        .filter(|(_, &p)| p == b"[[2]]" || p == b"[[6]]")
        .map(|(i, _)| i + 1)
        .product()
}
