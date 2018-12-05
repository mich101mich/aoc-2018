#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_5.txt");

    let mut min_c = 'a';
    let mut min = input.len();

    for c in 0..26 {
        let c = (c + 97u8) as char;
        let upper = c.to_uppercase().next().unwrap();
        let mut input = input
            .chars()
            .filter(|i| *i != c && *i != upper)
            .collect::<Vec<_>>();

        let mut change = true;
        while change {
            change = false;
            let mut i = 0;
            while i < input.len() - 1 {
                if input[i].is_lowercase() {
                    if input[i].to_uppercase().next().unwrap() == input[i + 1] {
                        input.remove(i);
                        input.remove(i);
                        change = true;
                        i -= 2;
                    }
                } else {
                    if input[i].to_lowercase().next().unwrap() == input[i + 1] {
                        input.remove(i);
                        input.remove(i);
                        change = true;
                        i -= 2;
                    }
                }
                i += 1;
            }
        }
        println!("{}: {}", c, input.len());
        if input.len() < min {
            min = input.len();
            min_c = c;
        }
    }
    println!("min: {}: {}", min_c, min);
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(width).collect::<Vec<T>>())
        .take(height)
        .collect()
}
