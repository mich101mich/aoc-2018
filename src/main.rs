#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_3.txt");

    let mut grid: Vec<Vec<usize>> = std::iter::repeat(
        std::iter::repeat(0).take(1000).collect::<Vec<usize>>(),
    ).take(1000)
        .collect();
    let mut count = 0;

    for line in input.lines() {
        let line = line.split(" @ ").skip(1).next().unwrap(); // 170,644: 29x14
        let x = usize::from_str(line.split(",").next().unwrap()).unwrap();
        let line = line.split(",").skip(1).next().unwrap();
        let y = usize::from_str(line.split(": ").next().unwrap()).unwrap();
        let size = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split("x")
            .collect::<Vec<&str>>();
        let width = usize::from_str(size[0]).unwrap();
        let height = usize::from_str(size[1]).unwrap();
        for dx in x..(x + width) {
            for dy in y..(y + height) {
                grid[dx][dy] += 1;
                if grid[dx][dy] == 2 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
