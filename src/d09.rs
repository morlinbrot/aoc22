use std::collections::HashSet;

const INPUT: &str = include_str!("./d09_input_ex");

pub const RENDER: bool = false;

pub fn part_one() -> usize {
    calculate_moves(2)
}

pub fn part_two() -> usize {
    calculate_moves(10)
}

pub fn calculate_moves(knots: usize) -> usize {
    let mut set = HashSet::new();
    let mut rope: Vec<(i64, i64)> = vec![(0, 0); knots];

    for line in INPUT.lines() {
        let (x, y, n) = match line.split_at(2) {
            ("R ", n) => (1, 0, n),
            ("L ", n) => (-1, 0, n),
            ("U ", n) => (0, 1, n),
            ("D ", n) => (0, -1, n),
            _ => unreachable!(),
        };

        for _ in 0..n.parse::<usize>().unwrap() {
            rope[0].0 += x;
            rope[0].1 += y;

            for i in 1..rope.len() {
                let (head, tail) = (rope[i - 1], rope[i]);
                let (dx, dy) = (tail.0 - head.0, tail.1 - head.1);
                let (dxl, dyl) = (dx.clamp(-1, 1), dy.clamp(-1, 1));

                rope[i] = match (dx.abs() > 1, dy.abs() > 1) {
                    (true, true) => (head.0 + dxl, head.1 + dyl),
                    (true, false) => (head.0 + dxl, head.1),
                    (false, true) => (head.0, head.1 + dyl),
                    _ => tail,
                };

                let last = rope.last().unwrap().clone();
                set.insert(last);
            }
        }
    }

    set.iter().count()
}

// Backup version with rendering.
// pub fn calculate_moves(knots: usize) -> usize {
//     let mut set = HashSet::new();
//     let mut rope: Vec<(i64, i64)> = vec![(0, 0); knots];

//     let names = vec!["0", "1", "2", "3", "4", "5", " 6", "7", "8", "9", "10"];
//     let grid_size = 30;

//     for line in INPUT.lines() {
//         let step = line.split(' ').collect::<Vec<_>>();

//         let (x, y, n) = match line.split_at(2) {
//             ("R ", n) => (1, 0, n),
//             ("L ", n) => (-1, 0, n),
//             ("U ", n) => (0, 1, n),
//             ("D ", n) => (0, -1, n),
//             _ => unreachable!(),
//         };

//         let n = n.parse::<usize>().unwrap();
//         for step_no in 0..n {
//             rope[0].0 += x;
//             rope[0].1 += y;

//             let mut grid = vec![vec!["."; grid_size as usize]; grid_size as usize];

//             for i in 1..rope.len() {
//                 let (head, tail) = (rope[i - 1], rope[i]);
//                 let dx = tail.0 - head.0;
//                 let dy = tail.1 - head.1;

//                 let (xcl, ycl) = (dx.clamp(-1, 1), dy.clamp(-1, 1));

//                 rope[i] = match (dx.abs() > 1, dy.abs() > 1) {
//                     (true, true) => (head.0 + xcl, head.1 + ycl),
//                     (true, false) => (head.0 + xcl, head.1),
//                     (false, true) => (head.0, head.1 + ycl),
//                     _ => (tail.0, tail.1),
//                 };

//                 let last = rope.last().unwrap().clone();
//                 set.insert(last);

//                 if RENDER {
//                     update_grid(&mut grid, rope[i], names[i]);

//                     if step_no == n - 1 && i == rope.len() - 1 {
//                         update_grid(&mut grid, rope[0], "H");
//                         render(&grid, &step);
//                     }
//                 }
//             }
//         }
//     }

//     set.iter().count()
// }

// fn update_grid(grid: &mut Vec<Vec<&str>>, pos: (i64, i64), char: &'static str) {
//     let x = pos.0 + 12;
//     let y = -pos.1 + 20;
//     grid[y as usize][x as usize] = char;
// }

// fn render(grid: &Vec<Vec<&str>>, step: &Vec<&str>) {
//     println!("\n");
//     println!("STEP: {:?}", step);
//     for row in grid.iter() {
//         println!("{:?}", row);
//     }
// }
