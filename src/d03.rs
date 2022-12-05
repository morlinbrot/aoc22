use std::collections::HashMap;

fn make_map() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn part_two() -> usize {
    let map = make_map();
    let str = std::fs::read_to_string("./src/d03_input.txt").unwrap();
    let groups = str.lines().collect::<Vec<&str>>();

    groups.chunks(3).fold(0, |sum, group| {
        let (one, two, three) = (group[0], group[1], group[2]);
        let c = one
            .chars()
            .find(|&c| two.contains(c) && three.contains(c))
            .unwrap();
        sum + map.get(&c).unwrap()
    })
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/d03_input.txt").unwrap();

    let map = make_map();
    str.lines().fold(0, |sum, line| {
        let (left, right) = line.split_at(line.len() / 2);
        let c = left.chars().find(|&c| right.contains(c)).unwrap();
        sum + map.get(&c).unwrap()
    })
}
