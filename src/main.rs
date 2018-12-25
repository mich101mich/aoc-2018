#![feature(vec_remove_item)]
#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use rand::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

#[macro_use]
mod utils;
use crate::utils::*;

pub fn manhattan_4d(p1: (isize, isize, isize, isize), p2: (isize, isize, isize, isize)) -> usize {
    diff_i(p1.0, p2.0) + diff_i(p1.1, p2.1) + diff_i(p1.2, p2.2) + diff_i(p1.3, p2.3)
}

fn main() {
    let input = include_str!("input/day_25.txt");

    let lines = input
        .lines()
        .map(|line| {
            let l = line
                .split(',')
                .map(|s| isize::from_str(s).unwrap())
                .collect::<Vec<_>>();
            (l[0], l[1], l[2], l[3])
        })
        .collect::<Vec<_>>();

    let mut constellations = vec![vec![lines[0]]];

    for &c in &lines {
        let mut all = vec![];
        for other in &constellations {
            let mut there = false;
            for p in other {
                if manhattan_4d(*p, c) <= 3 {
                    there = true;
                    break;
                }
            }
            if there {
                all.push(other.clone());
            }
        }
        let mut constel = vec![];
        for x in all {
            constellations.remove_item(&x);
            for p in x {
                constel.push(p);
            }
        }
        constel.push(c);
        constellations.push(constel);
    }
    pv!(constellations);
    pv!(constellations.len());
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
