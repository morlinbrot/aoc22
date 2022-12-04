// use crate::util;

// A - X - Rock - 1
// B - Y - Paper - 2
// C - Z - Scissors - 3

#[derive(Debug)]
pub enum Mv {
    Rck,
    Ppr,
    Scs,
}

impl Mv {
    fn against(&self, my_mv: &Mv) -> usize {
        match (self, my_mv) {
            (Mv::Rck, Mv::Ppr) | (Mv::Ppr, Mv::Scs) | (Mv::Scs, Mv::Rck) => 6,
            (Mv::Rck, Mv::Scs) | (Mv::Ppr, Mv::Rck) | (Mv::Scs, Mv::Ppr) => 0,
            _ => 3,
        }
    }

    fn val(&self) -> usize {
        match self {
            Mv::Rck => 1,
            Mv::Ppr => 2,
            Mv::Scs => 3,
        }
    }
}

pub fn part_one() -> usize {
    let str = std::fs::read_to_string("./src/day02_input.txt").unwrap();

    str.lines().fold(0, |sum, line| {
        let round: Vec<Mv> = line
            .split_whitespace()
            .map(|mv| match mv {
                "A" | "X" => Mv::Rck,
                "B" | "Y" => Mv::Ppr,
                "C" | "Z" => Mv::Scs,
                _ => unreachable!(),
            })
            .collect();

        let (elf, me) = (&round[0], &round[1]);
        sum + elf.against(me) + me.val()
    })
}
