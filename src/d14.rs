const INPUT: &str = include_str!("./d14_input");

// FIXME: All of this is quite ugly but I can't be bothered anymore.
const PADDING: usize = 1000;
const CORRECTION: usize = 0;

type Cave = Vec<Vec<char>>;

fn print_cave(cave: &Cave) {
    for (i, l) in cave.iter().enumerate() {
        println!("{:?}", (i, l));
    }
}

pub fn simulate(part_one: bool) -> usize {
    let mut coords = vec![];
    for line in INPUT.lines() {
        let splits = line.split([' ']).filter(|&s| s != "->").collect::<Vec<_>>();

        let c = splits
            .iter()
            .map(|s| s.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            // FIXME
            .map(|(x, y)| (x - CORRECTION, y))
            .collect::<Vec<_>>();

        coords.push(c);
    }

    let &y_bound = coords.iter().flatten().map(|(_, y)| y).max().unwrap();
    let &x_bound = coords.iter().flatten().map(|(x, _)| x).max().unwrap();

    let mut cave: Cave = vec![vec!['.'; x_bound + PADDING]; y_bound + 1];

    for (_, line) in coords.iter().enumerate() {
        for (i, crd) in line.iter().enumerate().skip(1) {
            let prev = line[i - 1];
            let cur = line[i];

            // We are switching x and y here to render correctly.
            let xs = prev.1.min(cur.1);
            let xl = prev.1.max(cur.1);
            let ys = prev.0.min(cur.0);
            let yl = prev.0.max(cur.0);

            for x in xs..=xl {
                for y in ys..=yl {
                    cave[x][y] = '#';
                }
            }
        }
    }

    let len = cave[0].len();
    let air = vec!['.'; len];
    cave.push(air);
    cave.push(vec!['#'; len]);

    let mut count = 0;

    let mut full = false;
    while !full {
        // FIXME
        let y = 500 - CORRECTION;

        let mut pos = (0, y);

        loop {
            // println!("LOOP {:?}", (pos));

            if part_one {
                if pos.0 >= cave.len() - 2 {
                    // println!("FULL {:?}", pos);
                    full = true;
                    break;
                }
            }

            let down = (pos.0 + 1, pos.1);
            if cave[down.0][down.1] == '.' {
                pos = down;
                continue;
            }

            let left = (pos.0 + 1, pos.1 - 1);
            if cave[left.0][left.1] == '.' {
                pos = left;
                continue;
            }

            let right = (pos.0 + 1, pos.1 + 1);
            if cave[right.0][right.1] == '.' {
                pos = right;
                continue;
            }

            cave[pos.0][pos.1] = 'o';
            count += 1;

            // print_cave(&cave);

            if pos == (0, 500 - CORRECTION) {
                full = true;
            }

            break;
        }
    }

    // print_cave(&cave);

    count
}

pub fn part_one() -> usize {
    simulate(true)
}

pub fn part_two() -> usize {
    simulate(false)
}
