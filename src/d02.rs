// use crate::util;

// Part 1
// A - X - Rock - Loss - 1
// B - Y - Paper - Draw - 2
// C - Z - Scissors - Win - 3

// For part 1, (A | X, B | Y, C | Z) maps to (Rck, Ppr, Scr) at values (1, 2, 3).
#[derive(Clone, Copy, Debug)]
pub enum Mv {
    Rck,
    Ppr,
    Scr,
}

impl Mv {
    // Produce the result for a given set of moves.
    fn against(&self, my_mv: &Mv) -> Res {
        match (self, my_mv) {
            (Mv::Rck, Mv::Ppr) | (Mv::Ppr, Mv::Scr) | (Mv::Scr, Mv::Rck) => Res::Win,
            (Mv::Rck, Mv::Scr) | (Mv::Ppr, Mv::Rck) | (Mv::Scr, Mv::Ppr) => Res::Loss,
            _ => Res::Draw,
        }
    }

    fn val(&self) -> usize {
        match self {
            Mv::Rck => 1,
            Mv::Ppr => 2,
            Mv::Scr => 3,
        }
    }

    // Produce the desired result for a given move.
    fn produce(&self, res: &Res) -> Mv {
        match (self, res) {
            (Mv::Rck, Res::Loss) | (Mv::Ppr, Res::Win) => Mv::Scr,
            (Mv::Ppr, Res::Loss) | (Mv::Scr, Res::Win) => Mv::Rck,
            (Mv::Scr, Res::Loss) | (Mv::Rck, Res::Win) => Mv::Ppr,

            (_, Res::Draw) => self.clone(),
        }
    }
}

impl From<&str> for Mv {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Mv::Rck,
            "B" | "Y" => Mv::Ppr,
            "C" | "Z" => Mv::Scr,
            _ => unreachable!(),
        }
    }
}

// For part 2, (X, Y, Z) maps to (Loss, Draw, Win) at values (1, 2, 3).
#[derive(Debug)]
enum Res {
    Loss,
    Draw,
    Win,
}

impl Res {
    fn val(&self) -> usize {
        match self {
            Res::Loss => 0,
            Res::Draw => 3,
            Res::Win => 6,
        }
    }
}

impl From<&Mv> for Res {
    fn from(mv: &Mv) -> Self {
        match mv {
            Mv::Rck => Res::Loss,
            Mv::Ppr => Res::Draw,
            Mv::Scr => Res::Win,
        }
    }
}

pub fn part_two() -> usize {
    let str = std::fs::read_to_string("./src/d02_input.txt").unwrap();

    str.lines().fold(0, |sum, line| {
        let round: Vec<Mv> = line.split_whitespace().map(Into::into).collect();
        let (mv, res) = (&round[0], Res::from(&round[1]));

        // Result of the battle plus value of the move I need to produce.
        sum + res.val() + mv.produce(&res).val()
    })
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/d02_input.txt").unwrap();

    str.lines().fold(0, |sum, line| {
        let round: Vec<Mv> = line.split_whitespace().map(Into::into).collect();
        let (elf, me) = (&round[0], &round[1]);
        // Result of the battle plus value of my move.
        sum + elf.against(me).val() + me.val()
    })
}
