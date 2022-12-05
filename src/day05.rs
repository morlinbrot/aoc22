use std::collections::HashMap;

type Stacks = HashMap<usize, Vec<char>>;
type Moves = Vec<Vec<usize>>;

pub fn parse_input() -> (Stacks, Moves) {
    let str = std::fs::read_to_string("./src/day05_input").unwrap();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut moves: Vec<Vec<usize>> = vec![];

    for line in str.lines() {
        if line.starts_with("move") {
            let splits: Vec<&str> = line.split(" ").collect();
            let set: Vec<usize> = splits.iter().filter_map(|elm| elm.parse().ok()).collect();
            moves.push(set);
        } else {
            for (i, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    stacks
                        .entry(i)
                        .and_modify(|vec| vec.push(c))
                        .or_insert(vec![c]);
                }
            }
        }
    }

    let mut sorted: Vec<_> = stacks.drain().collect();
    sorted.sort();

    for (i, s) in sorted.iter().cloned().enumerate() {
        let stack: Vec<_> = s.1.into_iter().rev().collect();
        stacks.insert(i + 1, stack);
    }

    (stacks, moves)
}

fn string_from_tops(stacks: Stacks) -> String {
    let mut sorted: Vec<_> = stacks.into_iter().collect();
    sorted.sort();
    sorted
        .into_iter()
        .fold(String::new(), |mut res, mut stack| {
            res.push(stack.1.pop().unwrap());
            res
        })
}

pub fn part_two() -> String {
    let (mut stacks, moves) = parse_input();

    for m in moves {
        let (count, from_no, to_no) = (m[0], m[1], m[2]);
        let from_stck = stacks.get_mut(&from_no).expect("Failed to find from_stck");
        let mut v: Vec<_> = from_stck.drain(from_stck.len() - count..).collect();

        let to_stck = stacks.get_mut(&to_no).expect("Failed to find to_stck");
        to_stck.append(&mut v);
    }

    string_from_tops(stacks)
}

pub fn part_one() -> String {
    let (mut stacks, moves) = parse_input();

    for m in moves {
        let (count, from_no, to_no) = (m[0], m[1], m[2]);
        for _ in 0..count {
            let from_stck = stacks.get_mut(&from_no).expect("Failed to find from_stck");
            let v = from_stck.pop();

            if let Some(v) = v {
                let stack = stacks.get_mut(&to_no).expect("Failed to find to_stck");
                stack.push(v);
            }
        }
    }

    string_from_tops(stacks)
}
