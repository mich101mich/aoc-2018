#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input: i32 = include!("input/day_11.txt");
    //let input = 42_i32;

    let mut grid = get_grid(0_i32, 300, 300);

    for y in 1..=300 {
        let mut row_sum = 0;
        for x in 1..=300 {
            let rack_id = x + 10;
            let mut pow = rack_id * y + input;
            pow *= rack_id;
            pow = (pow / 100) % 10 - 5;
            let (x, y) = (x as usize - 1, y as usize - 1);
            row_sum += pow;
            grid[x][y] = row_sum + if y > 0 { grid[x][y - 1] } else { 0 };
        }
    }
    let mut max = 0;
    let mut max_xy = (0, 0, 0);
    for size in 2..=300 {
        println!("{}", size);
        for y in size..300 {
            for x in size..300 {
                let sum =
                    grid[x][y] - grid[x - size][y] - grid[x][y - size] + grid[x - size][y - size];
                if sum > max {
                    max = sum;
                    max_xy = (x - size + 2, y - size + 2, size);
                }
            }
        }
    }
    println!("{}", max);
    println!("{},{},{}", max_xy.0, max_xy.1, max_xy.2);
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
