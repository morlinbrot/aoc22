const INPUT: &str = include_str!("./d10_input");

pub struct Device {
    pub x: i32,
    sprite: [i32; 3],
    cycle: usize,
    readings: Vec<i32>,
    screen: Vec<Vec<char>>,
}

impl Device {
    fn new() -> Self {
        Self {
            x: 1,
            sprite: [0, 1, 2],
            cycle: 0,
            readings: vec![],
            screen: vec![vec!['.'; 40]; 6],
        }
    }

    fn tick(&mut self, n: i32) {
        for _ in 0..n {
            self.draw();
            self.cycle += 1;
            for n in (20..=220).step_by(40) {
                if self.cycle == n {
                    self.readings.push(self.cycle as i32 * self.x);
                }
            }
        }
    }

    fn up_reg(&mut self, x: i32) {
        self.x += x;
    }

    fn up_sprite(&mut self) {
        let x = self.x;
        self.sprite = [x - 1, x, x + 1];
    }

    fn draw(&mut self) {
        let row = self.cycle / 40;
        let pixel = (self.cycle % 40) as i32;

        if self.sprite.contains(&pixel) {
            self.screen[row][pixel as usize] = '#';
        }
    }

    fn res(&self) -> i32 {
        self.readings.iter().sum()
    }

    fn res_v2(&self) {
        for line in &self.screen {
            let line = String::from_iter(line);
            println!("{:?}", line);
        }
    }
}

pub fn part_two() {
    let mut device = Device::new();

    for line in INPUT.lines() {
        let cmd = line.split(' ').collect::<Vec<_>>();

        match cmd[0] {
            "addx" => {
                device.tick(2);
                device.up_reg(cmd[1].parse::<i32>().unwrap());
                device.up_sprite();
            }
            "noop" => {
                device.tick(1);
            }
            _ => unreachable!(),
        }
    }

    device.res_v2()
}

pub fn part_one() -> i32 {
    let mut device = Device::new();

    for line in INPUT.lines() {
        let cmd = line.split(' ').collect::<Vec<_>>();

        match cmd[0] {
            "addx" => {
                device.tick(2);
                device.up_reg(cmd[1].parse::<i32>().unwrap());
            }
            "noop" => device.tick(1),
            _ => unreachable!(),
        }
    }

    // println!("{:?}", device.readings);

    device.res()
}
