use std::collections::HashMap;

pub fn parse_input() -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let str = std::fs::read_to_string("./src/day05_input").unwrap();

    let (raw_stacks, raw_moves) = str.split_once("\n\n").unwrap();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for line in raw_stacks.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks
                    .entry(i)
                    .and_modify(|vec| vec.push(c))
                    .or_insert(vec![c]);
            }
        }
    }

    let mut moves: Vec<Vec<usize>> = vec![];
    for line in raw_moves.lines() {
        let splits: Vec<&str> = line.split(" ").collect();
        let set: Vec<usize> = splits.iter().filter_map(|elm| elm.parse().ok()).collect();
        moves.push(set);
    }

    let mut sorted: Vec<_> = stacks.drain().collect();
    sorted.sort();

    let stacks: Vec<Vec<char>> = sorted
        .into_iter()
        .map(|(_, st)| st.into_iter().rev().collect())
        .collect();

    (stacks, moves)
}

fn string_from_tops(stacks: Vec<Vec<char>>) -> String {
    stacks.into_iter().fold(String::new(), |mut str, mut vec| {
        str.push(vec.pop().unwrap());
        str
    })
}

pub fn part_two() -> String {
    let (mut stacks, moves) = parse_input();

    for m in moves {
        let (count, from, to) = (m[0], m[1] - 1, m[2] - 1);

        let len = stacks[from].len();
        let mut v: Vec<_> = stacks[from].drain(len - count..).collect();

        stacks[to].append(&mut v);
    }

    string_from_tops(stacks)
}

pub fn part_one() -> String {
    let (mut stacks, moves) = parse_input();

    for m in moves {
        let (count, from, to) = (m[0], m[1] - 1, m[2] - 1);
        for _ in 0..count {
            let v = stacks[from].pop().expect("Failed to pop");
            stacks[to].push(v);
        }
    }

    string_from_tops(stacks)
}
