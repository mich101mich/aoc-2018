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

// taken from print_code and manually optimized
fn optimized() -> HashMap<usize, usize> {
    let mut b;
    let mut d = 0;
    let mut iter = 0;
    let mut seen = HashMap::new();

    loop {
        b = d | (1 << 16);
        d = 10373714;
        b *= 256;
        while b >= 256 {
            b /= 256;
            d = (((d + (b & 255)) & 16777215) * 65899) & 16777215;
        }

        match seen.entry(d) {
            Entry::Occupied(_) => break,
            Entry::Vacant(e) => {
                e.insert(iter);
            }
        }
        iter += 1;
    }
    seen
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let iter_count = optimized();
    let max = iter_count.iter().max_by_key(|(_, instr)| *instr).unwrap();
    pv!(max.0);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    // print_code(input);

    let iter_count = optimized();
    let min = iter_count.iter().min_by_key(|(_, instr)| *instr).unwrap();
    pv!(min.0);
}

#[allow(unused)]
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
