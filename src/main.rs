#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

struct Instruction {
    before: Vec<usize>,
    after: Vec<usize>,
    op: usize,
    l: usize,
    r: usize,
    out: usize,
}

fn main() {
    let input = include_str!("input/day_16.txt");

    let lines = input.lines().take(3260).collect::<Vec<_>>();
    let instructions = lines
        .chunks(4)
        .map(|inst| {
            let before = inst[0][9..19]
                .split(", ")
                .map(|i| usize::from_str(i).unwrap())
                .collect::<Vec<_>>();
            let after = inst[2][9..19]
                .split(", ")
                .map(|i| usize::from_str(i).unwrap())
                .collect::<Vec<_>>();
            let op = inst[1]
                .split(' ')
                .map(|i| usize::from_str(i).unwrap())
                .collect::<Vec<_>>();

            Instruction {
                before,
                after,
                op: op[0],
                l: op[1],
                r: op[2],
                out: op[3],
            }
        })
        .collect::<Vec<_>>();

    let opcodes = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];

    let mut possible = get_grid(true, 16, 16);

    let mut count = 0;

    for instr in instructions {
        let mut avail = 0;
        for (i, o) in opcodes.iter().enumerate() {
            match *o {
                "addr" => {
                    let res = instr.before[instr.l] + instr.before[instr.r];
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "addi" => {
                    let res = instr.before[instr.l] + instr.r;
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "mulr" => {
                    let res = instr.before[instr.l] * instr.before[instr.r];
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "muli" => {
                    let res = instr.before[instr.l] * instr.r;
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "banr" => {
                    let res = instr.before[instr.l] & instr.before[instr.r];
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "bani" => {
                    let res = instr.before[instr.l] & instr.r;
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "borr" => {
                    let res = instr.before[instr.l] | instr.before[instr.r];
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "bori" => {
                    let res = instr.before[instr.l] | instr.r;
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "setr" => {
                    let res = instr.before[instr.l];
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "seti" => {
                    let res = instr.l;
                    if res == instr.after[instr.out] {
                        avail += 1;
                    }
                }
                "gtir" => {
                    let res = instr.l > instr.before[instr.r];
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                "gtri" => {
                    let res = instr.before[instr.l] > instr.r;
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                "gtrr" => {
                    let res = instr.before[instr.l] > instr.before[instr.r];
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                "eqir" => {
                    let res = instr.l == instr.before[instr.r];
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                "eqri" => {
                    let res = instr.before[instr.l] == instr.r;
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                "eqrr" => {
                    let res = instr.before[instr.l] == instr.before[instr.r];
                    if res == (instr.after[instr.out] == 1) {
                        avail += 1;
                    }
                }
                _ => panic!("no op"),
            }
        }
        if avail >= 3 {
            count += 1;
        }
    }
    println!("{}", count);
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
