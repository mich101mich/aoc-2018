#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

#[allow(unused)]
macro_rules! pv {
    ($var: expr) => {
        println!("{}: {:?}", stringify!($var), $var)
    };
}

fn main() {
    let input = include_str!("input/day_22.txt");

    let (depth, target) = {
        let mut lines = input.lines();
        let d = usize::from_str(lines.next().unwrap().split(' ').nth(1).unwrap()).unwrap();
        let mut t = lines
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| usize::from_str(s).unwrap());

        (d, (t.next().unwrap(), t.next().unwrap()))
    };
    pv!(depth);
    pv!(target);

    let mut grid = get_grid(0, target.0 + 1, target.1 + 1);

    grid[0][0] = (0 + depth) % 20183;

    for y in 1..=target.1 {
        grid[0][y] = (y * 48271 + depth) % 20183;
    }
    for x in 1..=target.0 {
        grid[x][0] = (x * 16807 + depth) % 20183;
    }
    for x in 1..=target.0 {
        for y in 1..=target.1 {
            if x == target.0 && y == target.1 {
                grid[x][y] = (0 + depth) % 20183;
            } else {
                grid[x][y] = (grid[x - 1][y] * grid[x][y - 1] + depth) % 20183;
            }
        }
    }

    for x in 0..=target.0 {
        for y in 0..=target.1 {
            grid[x][y] %= 3;
        }
    }

    let mut sum = 0;
    for x in 0..=target.0 {
        for y in 0..=target.1 {
            sum += grid[x][y];
        }
    }
    pv!(sum);
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
