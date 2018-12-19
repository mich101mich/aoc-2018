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
    let input = include_str!("input/day_19.txt");

    let lines = input
        .lines()
        .skip(1)
        .map(|line| {
            let mut sp = line.split(' ');
            let op = sp.next().unwrap();
            let n = sp
                .map(|num| usize::from_str(num).unwrap())
                .collect::<Vec<_>>();
            (op, n[0], n[1], n[2])
        })
        .collect::<Vec<_>>();

    let ip_reg = usize::from_str(&input.lines().next().unwrap()[4..]).unwrap();
    println!("{}", ip_reg);

    let mut registers = [0; 6];

    let mut ip = 0;

    loop {
        registers[ip_reg] = ip;

        if ip >= lines.len() {
            println!("{}", registers[0]);
            return;
        }
        let instr = lines[ip];

        match instr.0 {
            "addr" => {
                let res = registers[instr.1] + registers[instr.2];
                registers[instr.3] = res;
            }
            "addi" => {
                let res = registers[instr.1] + instr.2;
                registers[instr.3] = res;
            }
            "mulr" => {
                let res = registers[instr.1] * registers[instr.2];
                registers[instr.3] = res;
            }
            "muli" => {
                let res = registers[instr.1] * instr.2;
                registers[instr.3] = res;
            }
            "banr" => {
                let res = registers[instr.1] & registers[instr.2];
                registers[instr.3] = res;
            }
            "bani" => {
                let res = registers[instr.1] & instr.2;
                registers[instr.3] = res;
            }
            "borr" => {
                let res = registers[instr.1] | registers[instr.2];
                registers[instr.3] = res;
            }
            "bori" => {
                let res = registers[instr.1] | instr.2;
                registers[instr.3] = res;
            }
            "setr" => {
                let res = registers[instr.1];
                registers[instr.3] = res;
            }
            "seti" => {
                let res = instr.1;
                registers[instr.3] = res;
            }
            "gtir" => {
                let res = instr.1 > registers[instr.2];
                registers[instr.3] = res as usize;
            }
            "gtri" => {
                let res = registers[instr.1] > instr.2;
                registers[instr.3] = res as usize;
            }
            "gtrr" => {
                let res = registers[instr.1] > registers[instr.2];
                registers[instr.3] = res as usize;
            }
            "eqir" => {
                let res = instr.1 == registers[instr.2];
                registers[instr.3] = res as usize;
            }
            "eqri" => {
                let res = registers[instr.1] == instr.2;
                registers[instr.3] = res as usize;
            }
            "eqrr" => {
                let res = registers[instr.1] == registers[instr.2];
                registers[instr.3] = res as usize;
            }
            op => panic!("no op: {}", op),
        }

        ip = registers[ip_reg];
        ip += 1;
    }
}
