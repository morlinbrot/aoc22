use std::fs;

pub fn read_and_split() -> Vec<usize> {
    let path = "./src/day01/input.txt";
    let contents = fs::read_to_string(path).unwrap();

    contents
        .split("\n")
        .fold((vec![], 0), |(mut res, sum), cur| {
            match cur.parse::<usize>() {
                Ok(num) => (res, sum + num),
                _ => {
                    res.push(sum);
                    (res, 0)
                }
            }
        })
        .0
}

pub fn part_two() -> usize {
    let mut res = read_and_split();
    res.sort_by(|a, b| b.cmp(a));
    res[0..3].iter().sum()
}

pub fn part_one() -> usize {
    let res = read_and_split();
    *res.iter().max().unwrap()
}

// pub fn part_one() -> usize {
//     let path = "./src/day01/input.txt";
//     let contents = fs::read_to_string(path).unwrap();

//     contents
//         .split("\n")
//         .fold((0, 0), |(res, sum), cur| match cur.parse::<usize>() {
//             Ok(num) => (res, sum + num),
//             _ => (res.max(sum), 0),
//         })
//         .0
// }
