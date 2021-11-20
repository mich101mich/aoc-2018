use crate::utils::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Opcode {
    Add(bool),
    Mul(bool),
    Ban(bool),
    Bor(bool),
    Set(bool),
    Gt(bool, bool),
    Eq(bool, bool),
}
use Opcode::*;

struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}
impl Instruction {
    #[allow(unused)]
    #[rustfmt::skip]
    const MAPPING: [Opcode; 16] = [Mul(true), Add(true), Ban(true), Eq(false, true), Mul(false), Set(true), Eq(true, false), Gt(true, false), Eq(true, true), Add(false), Gt(false, true), Gt(true, true), Bor(true), Ban(false), Set(false), Bor(false)];

    fn execute(&self, reg: &mut [usize], opcode: Opcode) {
        let (a, b, c) = (self.a, self.b, self.c);
        let get = |i, r| if r { reg[i] } else { i };

        match opcode {
            Add(r) => reg[c] = reg[a] + get(b, r),
            Mul(r) => reg[c] = reg[a] * get(b, r),
            Ban(r) => reg[c] = reg[a] & get(b, r),
            Bor(r) => reg[c] = reg[a] | get(b, r),
            Set(r) => reg[c] = get(a, r),
            Gt(r1, r2) => reg[c] = (get(a, r1) > get(b, r2)) as usize,
            Eq(r1, r2) => reg[c] = (get(a, r1) == get(b, r2)) as usize,
        }
    }
}
impl std::str::FromStr for Instruction {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        Ok(Instruction {
            opcode: iter.next().unwrap().parse()?,
            a: iter.next().unwrap().parse()?,
            b: iter.next().unwrap().parse()?,
            c: iter.next().unwrap().parse()?,
        })
    }
}
impl RegexRepresentation for Instruction {
    const REGEX: &'static str = r"\d+ \d+ \d+ \d+";
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = ;

    let mut iter = input.lines();

    #[rustfmt::skip]
    let mut opcodes = [Add(false), Add(true), Mul(false), Mul(true), Ban(false), Ban(true), Bor(false), Bor(true), Set(false), Set(true), Gt(false, true), Gt(true, false), Gt(true, true), Eq(false, true), Eq(true, false), Eq(true, true)];

    let mut possible = vec![vec![true; opcodes.len()]; opcodes.len()];

    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let before = scanf!(line, "Before: [{}]", String)
            .unwrap()
            .split(", ")
            .map(parse_u)
            .to_vec();
        let instr = Instruction::from_str(iter.next().unwrap()).unwrap();
        let after = scanf!(iter.next().unwrap(), "After:  [{}]", String)
            .unwrap()
            .split(", ")
            .map(parse_u)
            .to_vec();
        iter.next().unwrap();

        let opcode = instr.opcode;
        for (i, possible) in possible[opcode].iter_mut().enumerate() {
            if !*possible {
                continue;
            }
            let mut reg = before.clone();
            instr.execute(&mut reg, opcodes[i]);
            if reg != after {
                *possible = false;
            }
        }
    }

    let mut mapping = vec![Eq(false, false); opcodes.len()];
    for _ in 0..opcodes.len() {
        let (i, p) = possible
            .iter()
            .enumerate()
            .find(|(_, p)| p.iter().filter(|&&x| x).count() == 1)
            .unwrap();
        let j = p.iter().position(|&x| x).unwrap();
        mapping[i] = opcodes[j];
        possible.iter_mut().for_each(|p| p[j] = false);
    }
    assert!(mapping.iter().all(|x| *x != Eq(false, false)));

    pv!(mapping);

    iter.next().unwrap();
    let program = iter
        .map(Instruction::from_str)
        .filter_map(|r| r.ok())
        .to_vec();

    let mut reg = [0; 4];
    for instr in &program {
        instr.execute(&mut reg, mapping[instr.opcode]);
    }
    pv!(reg[0]);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = ;

    let mut iter = input.lines();

    #[rustfmt::skip]
    let mut opcodes = [Add(false), Add(true), Mul(false), Mul(true), Ban(false), Ban(true), Bor(false), Bor(true), Set(false), Set(true), Gt(false, true), Gt(true, false), Gt(true, true), Eq(false, true), Eq(true, false), Eq(true, true)];

    let mut like_tree = 0;
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let before = scanf!(line, "Before: [{}]", String)
            .unwrap()
            .split(", ")
            .map(parse_u)
            .to_vec();
        let instr = Instruction::from_str(iter.next().unwrap()).unwrap();
        let after = scanf!(iter.next().unwrap(), "After:  [{}]", String)
            .unwrap()
            .split(", ")
            .map(parse_u)
            .to_vec();
        iter.next().unwrap();

        let cnt = opcodes
            .iter()
            .filter(|opcode| {
                let mut reg = before.clone();
                instr.execute(&mut reg, **opcode);
                reg == after
            })
            .count();

        if cnt >= 3 {
            like_tree += 1;
        }
    }
    pv!(like_tree);
}
