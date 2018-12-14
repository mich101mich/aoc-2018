#![allow(unused_imports)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include!("input/day_14.txt");

    let mut recipies = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    while recipies.len() < input + 10 {
        let sum = recipies[e1] + recipies[e2];
        if sum >= 10 {
            recipies.push(sum / 10);
            recipies.push(sum % 10);
        } else {
            recipies.push(sum);
        }
        e1 = (e1 + recipies[e1] + 1) % recipies.len();
        e2 = (e2 + recipies[e2] + 1) % recipies.len();
        //println!("{:?}", recipies);
    }
    for n in &recipies[input..input + 10] {
        print!("{}", n);
    }
    println!();
}

#[allow(unused)]
fn manhatten(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
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
