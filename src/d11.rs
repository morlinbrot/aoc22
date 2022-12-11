use std::collections::HashMap;

const INPUT: &str = include_str!("./d11_input");

#[derive(Default, Debug)]
struct Monkey<'a> {
    id: usize,
    items: Vec<i64>,
    op: (&'a str, &'a str),
    test: i64,
    t_true: usize,
    t_false: usize,
    insp: usize,
}

fn parse_monkeys() -> HashMap<usize, Monkey<'static>> {
    let input = INPUT.split("\n\n");

    let mut monkeys = HashMap::new();
    for m in input {
        let mut monkey = Monkey::default();

        for line in m.lines() {
            let p: &[_] = &[':', ','];
            let splits = line
                .trim_start()
                .split_whitespace()
                .map(|s| s.trim_matches(p))
                .collect::<Vec<_>>();
            match splits[0] {
                "Monkey" => {
                    monkey.id = splits[1].parse::<usize>().unwrap();
                }
                "Starting" => {
                    monkey.items = splits
                        .iter()
                        .skip(2)
                        .map(|i| i.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                }
                "Operation" => {
                    monkey.op = (splits[4], splits[5]);
                }
                "Test" => {
                    monkey.test = splits[3].parse::<i64>().unwrap();
                }
                "If" => match splits[1] {
                    "true" => {
                        let to = splits[5].parse::<usize>().unwrap();
                        monkey.t_true = to;
                    }
                    "false" => {
                        let to = splits[5].parse::<usize>().unwrap();
                        monkey.t_false = to;
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        monkeys.insert(monkey.id, monkey);
    }

    monkeys
}

fn chase_monkeys(mut monkeys: HashMap<usize, Monkey>, rounds: usize, relief: bool) -> i64 {
    let len = monkeys.iter().count();

    // TODO: I have absolutely not yet really understood how this works.
    let lcm = monkeys.iter().map(|(_, m)| m.test).product::<i64>();

    for _ in 1..=rounds {
        for i in 0..len {
            let mut monkey = monkeys.remove(&i).unwrap();

            let items = monkey.items.drain(..);
            for item in items.into_iter() {
                monkey.insp += 1;
                let mut worry = item;

                let val = match monkey.op.1 {
                    "old" => worry,
                    val => val.parse::<i64>().unwrap(),
                };

                worry = match monkey.op.0 {
                    "+" => item + val,
                    "-" => item - val,
                    "*" => item * val,
                    "/" => item / val,
                    _ => unreachable!(),
                };

                if relief {
                    worry /= 3;
                } else {
                    worry = worry % lcm;
                }

                let to = match worry % monkey.test {
                    0 => monkey.t_true,
                    _ => monkey.t_false,
                };

                monkeys.get_mut(&to).unwrap().items.push(worry);
            }

            monkeys.insert(monkey.id, monkey);
        }
    }

    let mut res: Vec<i64> = monkeys.iter().map(|(_, m)| m.insp as i64).collect();
    res.sort_by_key(|i| -i);
    res.iter().take(2).product::<i64>()
}

pub fn part_one() -> i64 {
    let monkeys = parse_monkeys();
    chase_monkeys(monkeys, 20, true)
}

pub fn part_two() -> i64 {
    let monkeys = parse_monkeys();
    chase_monkeys(monkeys, 10_000, false)
}
