#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input: i32 = include!("input/day_11.txt");

    let mut grid = get_grid(0_i32, 300, 300);

    for y in 1..=300 {
        for x in 1..=300 {
            let rack_id = x + 10;
            let mut pow = rack_id * y + input;
            pow *= rack_id;
            pow = (pow / 100) % 10 - 5;
            grid[x as usize - 1][y as usize - 1] = pow;
        }
    }
    let mut max = 0;
    let mut max_xy = (0, 0);
    for y in 0..(300 - 3) {
        for x in 0..(300 - 3) {
            let sum = grid[x][y]
                + grid[x + 0][y + 1]
                + grid[x + 0][y + 2]
                + grid[x + 1][y + 0]
                + grid[x + 1][y + 1]
                + grid[x + 1][y + 2]
                + grid[x + 2][y + 0]
                + grid[x + 2][y + 1]
                + grid[x + 2][y + 2];
            if sum > max {
                max = sum;
                max_xy = (x + 1, y + 1);
            }
        }
    }
    println!("{},{}", max_xy.0, max_xy.1);
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
