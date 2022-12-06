use std::collections::HashSet;

fn find_signal<const L: usize>(chars: Vec<char>) -> usize {
    (0..chars.len())
        .skip(L - 1)
        .find_map(|i| {
            let mut set = HashSet::new();

            for j in 0..L {
                set.insert(chars[i - j]);
            }

            match set.len() {
                l if l == L => Some(i + 1),
                _ => None,
            }
        })
        .unwrap()
}

pub fn part_two() -> usize {
    let str = std::fs::read_to_string("./src/d06_input").unwrap();
    const LEN: usize = 14;
    // for line in str.lines() {
    //     println!("{:?}", find_signal::<LEN>(line.chars().collect()));
    // }

    find_signal::<LEN>(str.chars().collect())
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/d06_input").unwrap();
    const LEN: usize = 4;
    // for line in str.lines() {
    //     println!("{:?}", find_signal::<LEN>(line.chars().collect()));
    // }

    find_signal::<LEN>(str.chars().collect())
}
