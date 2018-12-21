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

    let mut prev = HashSet::new();
    let mut old = vec![];
    let mut logging = false;

    //for start in 0..1 {
        let mut registers = [0; 6];

        registers[0] = 16477902;
        let mut ip = 0;

        for round in 0..10000000000u64 {
            registers[ip_reg] = ip;
            if round == 2434104119 {
                println!("result {}", registers[3]);
                println!("{:?}", registers);
                println!("{:?}", old); // last value in this is last terminating value
                return;
            }
            if ip == 13 && registers[1] < 256 {
            if round > 2434104119 {
                println!("{:?}", registers);
                if round > 2434104219 {
                    return;
                }
            }
                //println!("{}", registers[3]); //2434104120 2434104120
                if !logging {
                    if !prev.insert(registers[3]) {
                        println!("{} in {}", registers[3], round);
                        logging = true;
                    }
                    if round > 2400004119 {
                        old.push(registers[3]);
                    }
                } else if prev.insert(registers[3]) {
                    println!("{}", registers[3]);
                }
            }
            /*if ip == 13 && registers[1] < 256 {
                logging = 10;
            }
            if logging > 0 {
                println!("{:?}", registers);
                logging -= 1;
                if logging == 0 {
                    return;
                }
            }*/
            //println!("{}, {}", round, ip);

            if ip >= lines.len() {
                println!("{} stops at {}", 16477902, round);
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

            /*if ip > 3 && instr.3 == 3 {
                println!("{:?}", registers[3]);
            }*/

            ip = registers[ip_reg];
            ip += 1;
        }
    
}
