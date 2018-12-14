#![allow(unused_imports)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include!("input/day_14.txt");

    let digits = {
        let mut d = vec![];
        let mut input = input;
        while input > 0 {
            d.insert(0, input % 10);
            input /= 10;
        }
        d
    };

    let mut recipies = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    loop {
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

        if recipies.len() < digits.len() + 1 {
            continue;
        }

        let mut found = true;
        for i in 0..digits.len() {
            if recipies[recipies.len() - digits.len() + i] != digits[i] {
                found = false;
                break;
            }
        }
        if found {
            println!("{}", recipies.len() - digits.len());
            return;
        }
        found = true;
        for i in 0..digits.len() {
            if recipies[recipies.len() - 1 - digits.len() + i] != digits[i] {
                found = false;
                break;
            }
        }
        if found {
            println!("{}", recipies.len() - 1 - digits.len());
            return;
        }
    }
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
