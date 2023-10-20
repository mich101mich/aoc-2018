use crate::utils::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromScanf)]
enum Reg {
    #[sscanf(format = "r")]
    Register,
    #[sscanf(format = "i")]
    Immediate,
}
impl Reg {
    fn get(&self, regs: &[usize], val: usize) -> usize {
        match self {
            Reg::Register => regs[val],
            Reg::Immediate => val,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromScanf)]
enum Opcode {
    #[sscanf(format = "add{}")]
    Add(Reg),
    #[sscanf(format = "mul{}")]
    Mul(Reg),
    #[sscanf(format = "ban{}")]
    Ban(Reg),
    #[sscanf(format = "bor{}")]
    Bor(Reg),
    #[sscanf(format = "set{}")]
    Set(Reg),
    #[sscanf(format = "gt{}{}")]
    Gt(Reg, Reg),
    #[sscanf(format = "eq{}{}")]
    Eq(Reg, Reg),
}
use Opcode::*;

#[derive(FromScanf)]
#[sscanf(format = "{opcode} {a} {b} {c}")]
struct Instruction {
    pub opcode: Opcode,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
impl Instruction {
    fn execute(&self, reg: &mut [usize]) {
        let (a, b, c) = (self.a, self.b, self.c);

        match self.opcode {
            Add(r) => reg[c] = reg[a] + r.get(reg, b),
            Mul(r) => reg[c] = reg[a] * r.get(reg, b),
            Ban(r) => reg[c] = reg[a] & r.get(reg, b),
            Bor(r) => reg[c] = reg[a] | r.get(reg, b),
            Set(r) => reg[c] = r.get(reg, a),
            Gt(r1, r2) => reg[c] = (r1.get(reg, a) > r2.get(reg, b)) as usize,
            Eq(r1, r2) => reg[c] = (r1.get(reg, a) == r2.get(reg, b)) as usize,
        }
    }
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
    let mut ip_reg = sscanf!(iter.next().unwrap(), "#ip {usize}").unwrap();
    let mut instructions = iter.map(|l| sscanf!(l, "{Instruction}").unwrap()).to_vec();

    let mut reg = vec![0; 6];
    while let Some(instr) = instructions.get(reg[ip_reg]) {
        instr.execute(&mut reg);
        reg[ip_reg] += 1;
    }
    pv!(reg[0]);
}

#[allow(unused)]
fn print_code(input: &str) {
    let mut iter = input.lines();
    let ip_reg = sscanf!(iter.next().unwrap(), "#ip {usize}").unwrap();
    let instructions = iter.map(|l| sscanf!(l, "{Instruction}").unwrap()).to_vec();

    for (i, instr) in instructions.iter().enumerate() {
        let to_str = |x: usize, r: Reg| {
            if r == Reg::Register {
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
        let c_reg = to_str(c, Reg::Register);
        let a_reg = to_str(a, Reg::Register);

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
