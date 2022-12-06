use std::collections::HashSet;

fn find_signal(chars: Vec<char>, len: usize) -> usize {
    (0..chars.len())
        .skip(len - 1)
        .find_map(|i| {
            let mut set = HashSet::new();

            for j in 0..len {
                set.insert(chars[i - j]);
            }

            match set.len() {
                l if l == len => Some(i + 1),
                _ => None,
            }
        })
        .unwrap()
}

pub fn part_two() -> usize {
    let str = std::fs::read_to_string("./src/d06_input").unwrap();
    let len = 14;
    // for line in str.lines() {
    //     println!("{:?}", find_signal(line.chars().collect(), len));
    // }

    find_signal(str.chars().collect(), len)
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/d06_input").unwrap();
    let len = 4;
    // for line in str.lines() {
    //     println!("{:?}", find_signal(line.chars().collect(), len));
    // }

    find_signal(str.chars().collect(), len)
}
