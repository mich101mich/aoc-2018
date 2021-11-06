use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let mut lines = input.lines().to_vec();
    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        'outer: for other in lines[i + 1..].iter() {
            result.clear();
            let mut found_difference = false;
            for (a, b) in line.chars().zip(other.chars()) {
                if a == b {
                    result.push(a);
                } else {
                    if found_difference {
                        continue 'outer;
                    }
                    found_difference = true;
                }
            }
            if found_difference {
                pv!(result);
                return;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut counts = HashMap::new();
        for c in line.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut has_two = false;
        let mut has_three = false;
        for c in counts.values() {
            has_two |= *c == 2;
            has_three |= *c == 3;
        }
        twos += has_two as u32;
        threes += has_three as u32;
    }
    pv!(twos * threes);
}
