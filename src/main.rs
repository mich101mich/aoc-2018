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

    let mut grid = get_grid(-1, h as usize * 5, w as usize * 5);

    let mut count = HashMap::new();
    for i in 0..points.len() {
        count.insert(i, 0);
    }

    for y in 0..h * 3 {
        for x in 0..w * 3 {
            let mut dists = points
                .iter()
                .enumerate()
                .map(|(i, p)| (i as i32, manhatten(*p, (x - w * 2, y - h * 2))))
                .collect::<Vec<_>>();
            dists.sort_by_key(|&(_, d)| d);
            if dists[0].1 == dists[1].1 {
                continue;
            }
            grid[y as usize][x as usize] = dists[0].0;
            let best = dists[0].0 as usize;
            if x == 0 || y == 0 || x == w * 4 - 1 || y == h * 4 - 1 {
                count.remove(&best);
            }
            if count.contains_key(&best) {
                *count.get_mut(&best).unwrap() += 1;
            }
        }
    }

    let max = *count.values().max().unwrap();
    println!("{}", max);
    println!("{:?}", count);
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
