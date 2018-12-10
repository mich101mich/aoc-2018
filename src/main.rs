#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_10.txt");

    let mut points = input
        .lines()
        .map(|line| {
            let x = i32::from_str(&line[10..16].trim()).unwrap();
            let y = i32::from_str(&line[18..24].trim()).unwrap();
            let dx = i32::from_str(&line[36..38].trim()).unwrap();
            let dy = i32::from_str(&line[40..42].trim()).unwrap();
            ((x, y), (dx, dy))
        })
        .collect::<Vec<_>>();

    let mut x_dist = 100_000_000;
    let mut end = 0;

    for i in 0..100_000 {
        let mut min_x = 10000;
        let mut max_x = -10000;
        let mut min_y = 10000;
        let mut max_y = -10000;
        for (p, _) in &points {
            if p.0 > max_x {
                max_x = p.0;
            }
            if p.0 < min_x {
                min_x = p.0;
            }
            if p.1 > max_y {
                max_y = p.1;
            }
            if p.1 < min_y {
                min_y = p.1;
            }
        }
        let mut needed = i >= 10864;

        let dist = max_x - min_x;
        if dist < x_dist {
            x_dist = dist;
        } else {
            println!("{}", i);
            needed = true;
        }

        if needed {
            println!("{}, {}, {}, {}", min_x, max_x, min_y, max_y);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let mut found = false;
                    for (p, _) in &points {
                        if p.0 == x && p.1 == y {
                            found = true;
                        }
                    }
                    if found {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
            println!();
            end += 1;
            if end > 5 {
                return;
            }
        }

        for (ref mut p, ref v) in &mut points {
            p.0 += v.0;
            p.1 += v.1;
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
