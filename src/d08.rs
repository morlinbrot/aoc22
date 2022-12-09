const INPUT: &str = include_str!("./d08_input");

fn parse_grid() -> Vec<Vec<u32>> {
    let mut grid = vec![];

    for line in INPUT.lines() {
        let row = line
            .chars()
            .map(|v| v.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        grid.push(row);
    }

    grid
}

enum Dir {
    Left,
    Up,
    Right,
    Down,
}

fn visible(dir: Dir, grid: &Vec<Vec<u32>>, height: u32, mut i: usize, mut j: usize) -> bool {
    if i == grid.len() - 1 || i == 0 || j == grid[i].len() - 1 || j == 0 {
        return true;
    }

    match dir {
        Dir::Left => j -= 1,
        Dir::Up => i -= 1,
        Dir::Right => j += 1,
        Dir::Down => i += 1,
    }

    if grid[i][j] >= height {
        return false;
    }

    visible(dir, grid, height, i, j)
}

fn view_dist(dir: Dir, grid: &Vec<Vec<u32>>, height: u32, mut i: usize, mut j: usize) -> usize {
    match dir {
        Dir::Left => {
            if j == 0 {
                return 0;
            }
            j -= 1
        }
        Dir::Up => {
            if i == 0 {
                return 0;
            }
            i -= 1
        }
        Dir::Right => {
            if j == grid[i].len() - 1 {
                return 0;
            }
            j += 1
        }
        Dir::Down => {
            if i == grid.len() - 1 {
                return 0;
            }
            i += 1
        }
    }

    if grid[i][j] >= height {
        return 1;
    }

    1 + view_dist(dir, grid, height, i, j)
}

pub fn part_two() -> usize {
    let grid = parse_grid();

    let mut scores = vec![vec![0; grid[0].len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for j in 0..row.len() {
            scores[i][j] = view_dist(Dir::Left, &grid, grid[i][j], i, j)
                * view_dist(Dir::Up, &grid, grid[i][j], i, j)
                * view_dist(Dir::Right, &grid, grid[i][j], i, j)
                * view_dist(Dir::Down, &grid, grid[i][j], i, j);
        }
    }

    scores.into_iter().flatten().max().unwrap()
}

pub fn part_one() -> usize {
    let grid = parse_grid();

    grid.iter().enumerate().fold(0, |sum, (i, row)| {
        sum + row.iter().enumerate().fold(0, |mut sum, (j, &tree)| {
            if visible(Dir::Left, &grid, tree, i, j)
                || visible(Dir::Up, &grid, tree, i, j)
                || visible(Dir::Right, &grid, tree, i, j)
                || visible(Dir::Down, &grid, tree, i, j)
            {
                sum += 1;
            }
            sum
        })
    })
}

// Original version
// pub fn part_one() -> usize {
//     let grid = parse_grid();

//     let mut count = 0;

//     for (i, row) in grid.iter().enumerate() {
//         for j in 0..row.len() {
//             if check_visibility(Dir::Left, &grid, grid[i][j], i, j)
//                 || check_visibility(Dir::Up, &grid, grid[i][j], i, j)
//                 || check_visibility(Dir::Right, &grid, grid[i][j], i, j)
//                 || check_visibility(Dir::Down, &grid, grid[i][j], i, j)
//             {
//                 count += 1;
//             }
//         }
//     }

//     count
// }
