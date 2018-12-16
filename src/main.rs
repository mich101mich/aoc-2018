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
    let input = include_str!("input/day_16.txt");

    let instructions = input
        .lines()
        .skip(3262)
        .map(|inst| {
            let op = inst
                .split(' ')
                .map(|i| usize::from_str(i).unwrap())
                .collect::<Vec<_>>();
            (op[0], op[1], op[2], op[3])
        })
        .collect::<Vec<_>>();

    let mapping: HashMap<usize, &str> = [
        (0_usize, "mulr"),
        (1, "addr"),
        (2, "banr"),
        (3, "eqir"),
        (4, "muli"),
        (5, "setr"),
        (6, "eqri"),
        (7, "gtri"),
        (8, "eqrr"),
        (9, "addi"),
        (10, "gtir"),
        (11, "gtrr"),
        (12, "borr"),
        (13, "bani"),
        (14, "seti"),
        (15, "bori"),
    ]
    .into_iter()
    .cloned()
    .collect();

    let mut registers = [0; 4];

    for instr in instructions {
        match mapping[&instr.0] {
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
            _ => panic!("no op"),
        }
    }
    println!("{}", registers[0]);
}

#[allow(unused)]
fn neighbours(
    (x, y): (usize, usize),
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut n = vec![];
    if y > 0 {
        n.push((x, y - 1));
    }
    if x > 0 {
        n.push((x - 1, y));
    }
    if x < width - 1 {
        n.push((x + 1, y));
    }
    if y < height - 1 {
        n.push((x, y + 1));
    }
    n.into_iter()
}

#[allow(unused)]
fn manhatten(p1: (usize, usize), p2: (usize, usize)) -> usize {
    ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as usize
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(height).collect::<Vec<T>>())
        .take(width)
        .collect()
}

#[allow(unused)]
#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
