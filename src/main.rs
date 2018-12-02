#![allow(unused_imports)]

use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input/day_2.txt");
    
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut count = HashMap::<char, usize>::new();
        for c in line.chars() {
            if count.contains_key(&c) {
                *count.get_mut(&c).unwrap() += 1;
            } else {
                count.insert(c, 1);
            }
        }
        let mut found_two = false;
        let mut found_three = false;
        for i in count.values() {
            if *i == 2 {
                if !found_two {
                    twos += 1;
                    found_two = true;
                }
            } else if *i == 3 {
                if !found_three {
                    threes += 1;
                    found_three = true;
                }
            }
        }
    }
    println!("{}", twos);
    println!("{}", threes);
    println!("{}", twos * threes);
}
