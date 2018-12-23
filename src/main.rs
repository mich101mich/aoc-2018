#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

#[macro_use]
mod utils;
use crate::utils::*;

fn main() {
    let input = include_str!("input/day_23.txt");

    let bots = input
        .lines()
        .map(|line| {
            let mut split = line[5..].split(">, r=");
            let pos = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| isize::from_str(s).unwrap())
                .collect::<Vec<_>>();
            let radius = usize::from_str(split.next().unwrap()).unwrap();
            ((pos[0], pos[1], pos[2]), radius)
        })
        .collect::<Vec<_>>();
    let mut radius = bots[0].1;
    let mut max_i = 0;
    for (i, (_, r)) in bots.iter().enumerate() {
        if *r > radius {
            max_i = i;
            radius = *r;
        }
    }
    let pos = bots[max_i].0;
    let mut count = 0;
    for (p, _) in &bots {
        if manhattan_3d_i(pos, *p) <= radius {
            count += 1;
        }
    }
    pv!(count);
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
