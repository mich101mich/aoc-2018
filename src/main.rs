#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

fn parse_range(range: &str) -> (usize, usize) {
    if range.contains("..") {
        let mut r = range.split("..").map(|s| usize::from_str(s).unwrap());
        (r.next().unwrap(), r.next().unwrap())
    } else {
        let r = usize::from_str(range).unwrap();
        (r, r)
    }
}

fn solid(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    grid[x][y] == '#' || grid[x][y] == '~'
}

fn main() {
    let input = include_str!("input/day_17.txt");

    let parsed = input
        .lines()
        .map(|line| {
            let l = line.split(", ").collect::<Vec<_>>();
            if l[0].starts_with('x') {
                (parse_range(&l[0][2..]), parse_range(&l[1][2..]))
            } else {
                (parse_range(&l[1][2..]), parse_range(&l[0][2..]))
            }
        })
        .collect::<Vec<_>>();
    let mut min_x = 500;
    let mut min_y = 0;
    let mut max_x = 500;
    let mut max_y = 0;
    for (xr, yr) in &parsed {
        if xr.0 < min_x {
            min_x = xr.0;
        }
        if xr.1 > max_x {
            max_x = xr.1;
        }
        if yr.0 < min_y {
            min_y = yr.0;
        }
        if yr.0 > max_y {
            max_y = yr.1;
        }
    }
    min_x -= 2;
    max_x += 2;
    println!("{}", min_x);
    println!("{}", min_y);
    println!("{}", max_x);
    println!("{}", max_y);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = get_grid('.', width, height);
    for (xr, yr) in &parsed {
        for y in yr.0..=yr.1 {
            for x in xr.0..=xr.1 {
                grid[x - min_x][y - min_y] = '#';
            }
        }
    }
    grid[500 - min_x][0 - min_y] = '+';

    let mut water = vec![(500 - min_x, 0)];

    let mut change = true;

    while change {
        change = false;
        for i in 0..water.len() {
            let (x, y) = water[i];
            if y + 1 == height || grid[x][y + 1] == '|' {
                continue;
            }

            let mut dx = 0;
            let mut dy = 0;
            while y + dy + 1 < height && grid[x][y + dy + 1] == '.' {
                dy += 1;
                grid[x][y + dy] = '|';
                water.push((x, y + dy));
                change = true;
            }
            if y + dy + 1 == height || grid[x][y + dy + 1] == '|' {
                continue;
            }

            while x - (dx + 1) > 0
                && grid[x - (dx + 1)][y + dy] == '.'
                && solid(&grid, x - dx, y + dy + 1)
            {
                dx += 1;
                grid[x - dx][y + dy] = '|';
                water.push((x - dx, y + dy));
                change = true;
            }

            dx = 0;

            while x + dx + 1 < width - 1
                && grid[x + dx + 1][y + dy] == '.'
                && solid(&grid, x + dx, y + dy + 1)
            {
                dx += 1;
                grid[x + dx][y + dy] = '|';
                water.push((x + dx, y + dy));
                change = true;
            }

            if grid[x][y + dy] == '|' && solid(&grid, x, y + dy + 1) && grid[x - 1][y + dy] == '#' {
                dx = 0;
                while x + dx + 1 < width
                    && !solid(&grid, x + dx + 1, y + dy)
                    && solid(&grid, x + dx, y + dy + 1)
                {
                    dx += 1;
                }
                if solid(&grid, x + dx + 1, y + dy) && solid(&grid, x + dx, y + dy + 1) {
                    for gx in 0..=dx {
                        grid[x + gx][y + dy] = '~';
                    }
                    change = true;
                }
            }
        }
        water.retain(|&(x, y)| grid[x][y] == '|');
        //for y in 0..height {
        //    for x in 0..width {
        //        print!("{}", grid[x][y]);
        //    }
        //    println!();
        //}
        //println!();
    }
    println!("done");
    let mut file = std::fs::File::create("out.txt").unwrap();
    for y in 0..height {
        for x in 0..width {
            write!(file, "{}", grid[x][y]).unwrap();
        }
        writeln!(file).unwrap();
    }
    writeln!(file).unwrap();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let c = grid[x][y];
            if c == '~' {
                count += 1;
            }
        }
    }
    println!("count: {}", count);
}
