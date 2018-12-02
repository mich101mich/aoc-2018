#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_2.txt");

    for line1 in input.lines() {
        for line2 in input.lines() {
            let mut diff = 0;
            for (c1, c2) in line1.chars().zip(line2.chars()) {
                if c1 != c2 {
                    diff += 1;
                }
            }
            if diff != 1 {
                continue;
            }
            println!("{}", line1);
            println!("{}", line2);
            return;
        }
    }
}
