pub fn part_two() -> usize {
    let str = std::fs::read_to_string("./src/d04_input").unwrap();
    str.lines()
        .map(|line| {
            line.split([',', '-'])
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|nums| !(nums[1] < nums[2] || nums[3] < nums[0]))
        .count()
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/d04_input").unwrap();
    str.lines()
        .map(|line| {
            line.split([',', '-'])
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|nums| {
            nums[0] >= nums[2] && nums[1] <= nums[3] || nums[0] <= nums[2] && nums[1] >= nums[3]
        })
        .count()
}
