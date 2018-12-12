#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_12.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let initial = &lines[0][15..];
    let lines = &lines[2..];

    let mut patterns = HashMap::new();
    for line in lines {
        patterns.insert(&line[0..5], line.chars().nth(9).unwrap());
    }

    let mut pots = HashMap::new();

    for (i, c) in initial.chars().enumerate() {
        if c == '#' {
            pots.insert(i as i32, c);
        }
    }

    let mut first: i32 = *pots.keys().min().unwrap();
    let mut last: i32 = *pots.keys().max().unwrap();

    let mut prev: i32 = pots.keys().sum();

    for gen in 0..2000 {
        let old = pots.clone();
        for p in (first - 4)..(last + 4) {
            let pat: &str = &format!(
                "{}{}{}{}{}",
                old.get(&(p - 2)).unwrap_or(&'.'),
                old.get(&(p - 1)).unwrap_or(&'.'),
                old.get(&p).unwrap_or(&'.'),
                old.get(&(p + 1)).unwrap_or(&'.'),
                old.get(&(p + 2)).unwrap_or(&'.')
            );
            if patterns[pat] == '#' {
                pots.insert(p, '#');
                if p < first {
                    first = p;
                }
                if p > last {
                    last = p;
                }
            } else {
                pots.remove(&p);
            }
        }
        if gen % 100 == 0 {
            let curr: i32 = pots.keys().sum();
            println!("{}", curr - prev);
            prev = curr;
        }
    }
    let cnt: i32 = pots.keys().sum();
    println!("{}", cnt);

    // 2000 => 158467
    // alle 100: + 7800
    // (50000000000 - 2000) / 100 * 7800
    // => 3899999844000
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
