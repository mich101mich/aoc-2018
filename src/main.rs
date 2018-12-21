#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

fn main() {
    let input = include_str!("input/day_21.txt");

}

#[allow(unused)]
fn assembler(input: &str) {
    let (lines, ip_reg) = parse_asm(input);

    let mut registers = [0; 6];

    let mut ip = 0;

    for round in 0_usize.. {

        if ip >= lines.len() {
            println!("stops at {}", round);
            return;
        }
        
        asm_run(lines[ip], &mut registers);

        ip = registers[ip_reg];
        ip += 1;
    }
}
