#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_5.txt");
    let mut input = input.chars().collect::<Vec<_>>();

    let mut change = true;
    while change {
        change = false;
        for i in 0..(input.len() - 1) {
            if input[i].is_lowercase() {
                if input[i].to_uppercase().next().unwrap() == input[i + 1] {
                    input.remove(i);
                    input.remove(i);
                    change = true;
                    break;
                }
            } else {
                if input[i].to_lowercase().next().unwrap() == input[i + 1] {
                    input.remove(i);
                    input.remove(i);
                    change = true;
                    break;
                }
            }
        }
    }
    println!("{}", input.len());
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(width).collect::<Vec<T>>())
        .take(height)
        .collect()
}
