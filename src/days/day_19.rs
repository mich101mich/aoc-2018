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
impl std::str::FromStr for Opcode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().to_vec();
        let a = chars[2] == 'r';
        let b = chars[3] == 'r';
        let op = match &s[..2] {
            "ad" => Add(b),
            "mu" => Mul(b),
            "ba" => Ban(b),
            "bo" => Bor(b),
            "se" => Set(b),
            "gt" => Gt(a, b),
            "eq" => Eq(a, b),
            _ => return Err(format!("Unknown opcode: {}", s)),
        };
        Ok(op)
    }
}

struct Instruction {
    pub opcode: Opcode,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
impl Instruction {
    fn execute(&self, reg: &mut [usize]) {
        let (a, b, c) = (self.a, self.b, self.c);
        let get = |i, r| if r { reg[i] } else { i };

        match self.opcode {
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
            opcode: iter.next().unwrap().parse().unwrap(),
            a: iter.next().unwrap().parse()?,
            b: iter.next().unwrap().parse()?,
            c: iter.next().unwrap().parse()?,
        })
    }
}
impl RegexRepresentation for Instruction {
    const REGEX: &'static str = r"[a-z]+ \d+ \d+ \d+";
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    // println!(" -1: a = 1;");
    // print_code(input);

    // optimized down to:
    let reg_5 = 10551387;
    let limit = (reg_5 as f64).sqrt().ceil() as usize;
    let reg_0 = (1..=limit)
        .filter(|reg_1| reg_5 % reg_1 == 0)
        .map(|reg_1| reg_1 + reg_5 / reg_1)
        .sum::<usize>();
    pv!(reg_0);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");
    // let input = ;

    let mut iter = input.lines();
    let mut ip_reg = scanf!(iter.next().unwrap(), "#ip {}", usize).unwrap();
    let mut instructions = iter.map(|l| l.parse::<Instruction>().unwrap()).to_vec();

    let mut reg = vec![0; 6];
    while let Some(instr) = instructions.get(reg[ip_reg]) {
        instr.execute(&mut reg);
        reg[ip_reg] += 1;
    }
    pv!(reg[0]);
}

#[allow(unused)]
fn print_code(input: &str) {
    fn print_code(input: &str) {
        let mut iter = input.lines();
        let ip_reg = scanf!(iter.next().unwrap(), "#ip {}", usize).unwrap();
        let instructions = iter.map(|l| l.parse::<Instruction>().unwrap()).to_vec();

        for (i, instr) in instructions.iter().enumerate() {
            let to_str = |x: usize, r| {
                if r {
                    if x == ip_reg {
                        i.to_string()
                    } else {
                        format!("{}", (b'a' + x as u8) as char)
                    }
                } else {
                    x.to_string()
                }
            };

            print!("{:>3}: ", i);

            let (a, b, c) = (instr.a, instr.b, instr.c);
            let c_reg = to_str(c, true);
            let a_reg = to_str(a, true);

            match instr.opcode {
                Add(r) => {
                    if c == ip_reg {
                        if a == ip_reg {
                            println!("Jump by {} + 1", to_str(b, r));
                        } else {
                            println!("Jump to {} + {} + 1", a_reg, to_str(b, r));
                        }
                    } else if c_reg == a_reg {
                        println!("{} += {}", c_reg, to_str(b, r));
                    } else if c_reg == to_str(b, r) {
                        println!("{} += {}", c_reg, a_reg);
                    } else {
                        println!("{} = {} + {}", c_reg, a_reg, to_str(b, r));
                    }
                }
                Mul(r) => {
                    if c == ip_reg {
                        if a == ip_reg {
                            println!("Jump to {} * {} + 1", i, to_str(b, r));
                        } else {
                            println!("Jump to {} * {} + 1", a_reg, to_str(b, r));
                        }
                    } else if c_reg == a_reg {
                        println!("{} *= {}", c_reg, to_str(b, r));
                    } else if c_reg == to_str(b, r) {
                        println!("{} *= {}", c_reg, a_reg);
                    } else {
                        println!("{} = {} * {}", c_reg, a_reg, to_str(b, r));
                    }
                }
                Ban(r) => {
                    if c_reg == a_reg {
                        println!("{} &= {}", c_reg, to_str(b, r));
                    } else if c_reg == to_str(b, r) {
                        println!("{} &= {}", c_reg, a_reg);
                    } else {
                        println!("{} = {} & {}", c_reg, a_reg, to_str(b, r));
                    }
                }
                Bor(r) => {
                    if c_reg == a_reg {
                        println!("{} |= {}", c_reg, to_str(b, r));
                    } else if c_reg == to_str(b, r) {
                        println!("{} |= {}", c_reg, a_reg);
                    } else {
                        println!("{} = {} | {}", c_reg, a_reg, to_str(b, r));
                    }
                }
                Set(r) => {
                    if c == ip_reg {
                        println!("Jump to {} + 1", to_str(a, r));
                    } else {
                        println!("{} = {}", c_reg, to_str(a, r));
                    }
                }
                Gt(r1, r2) => println!("{} = {} > {}", c_reg, to_str(a, r1), to_str(b, r2)),
                Eq(r1, r2) => println!("{} = {} == {}", c_reg, to_str(a, r1), to_str(b, r2)),
            }
        }
    }
}
