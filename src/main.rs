#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_6.txt");

    let mut w = 0;
    let mut h = 0;

    let points = input
        .lines()
        .map(|line| {
            let mut l = line.split(", ").map(|s| i32::from_str(s).unwrap());
            (l.next().unwrap(), l.next().unwrap())
        })
        .collect::<Vec<_>>();
    for p in points.iter() {
        if p.0 > w {
            w = p.0;
        }
        if p.1 > h {
            h = p.1;
        }
    }

    let mut count = 0;

    for y in 0..h * 5 {
        for x in 0..w * 5 {
            let dist: i32 = points
                .iter()
                .map(|p| manhatten(*p, (x - w * 2, y - h * 2)))
                .sum();
            if dist < 10000 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn manhatten(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(height).collect::<Vec<T>>())
        .take(width)
        .collect()
}
