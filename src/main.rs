#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

#[allow(unused)]
macro_rules! pv {
    ($var: expr) => {
        println!("{}: {:?}", stringify!($var), $var)
    };
}

fn main() {
    let input = include_str!("input/day_22.txt");

    let (depth, target) = {
        let mut lines = input.lines();
        let d = usize::from_str(lines.next().unwrap().split(' ').nth(1).unwrap()).unwrap();
        let mut t = lines
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| usize::from_str(s).unwrap());

        (d, (t.next().unwrap(), t.next().unwrap()))
    };
    pv!(depth);
    pv!(target);

    let w = target.0 + 500;
    let h = target.1 + 500;

    let mut grid = get_grid(0, w, h);

    for y in 0..h {
        grid[0][y] = (y * 48271 + depth) % 20183;
    }
    for x in 1..w {
        grid[x][0] = (x * 16807 + depth) % 20183;
    }
    for x in 1..w {
        for y in 1..h {
            if x == target.0 && y == target.1 {
                grid[x][y] = (0 + depth) % 20183;
            } else {
                grid[x][y] = (grid[x - 1][y] * grid[x][y - 1] + depth) % 20183;
            }
        }
    }

    for x in 0..w {
        for y in 0..h {
            grid[x][y] %= 3;
        }
    }

    let mut sum = 0;
    for x in 0..=target.0 {
        for y in 0..=target.1 {
            sum += grid[x][y];
        }
    }
    pv!(sum);

    fn get_tool(t: usize) -> [usize; 2] {
        match t {
            0 => [1, 2],
            1 => [2, 0],
            2 => [1, 0],
            c => panic!("{}", c),
        }
    }

    let path = a_star_search(
        |(x, y, tool)| {
            let mut targets = vec![];
            let tools = get_tool(grid[x][y]);
            if tool == tools[0] {
                targets.push((x, y, tools[1]));
            } else {
                targets.push((x, y, tools[0]));
            }
            if x > 0 {
                let next = (x - 1, y, tool);
                if get_tool(grid[next.0][next.1]).contains(&tool) {
                    targets.push(next);
                }
            }
            if y > 0 {
                let next = (x, y - 1, tool);
                if get_tool(grid[next.0][next.1]).contains(&tool) {
                    targets.push(next);
                }
            }
            if x < w - 1 {
                let next = (x + 1, y, tool);
                if get_tool(grid[next.0][next.1]).contains(&tool) {
                    targets.push(next);
                }
            }
            if y < h - 1 {
                let next = (x, y + 1, tool);
                if get_tool(grid[next.0][next.1]).contains(&tool) {
                    targets.push(next);
                }
            }
            targets.into_iter()
        },
        |a, b| if a.2 != b.2 { 7 } else { 0 } + diff(a.0, b.0) + diff(a.1, b.1),
        |_| true,
        (0, 0, 1),
        (target.0, target.1, 1),
        |p| manhattan((p.0, p.1), target),
    )
    .expect("no path in area");

    pv!(path.cost);
}

#[allow(unused)]
fn assembler(input: &str) {
    let (lines, ip_reg) = parse_asm(input);

    let mut registers = [0; 6];

    let mut ip = 0;

    for round in 0_usize.. {
        if ip >= lines.len() {
            println!("stops at {}", round);
            return;
        }

        asm_run(lines[ip], &mut registers);

        ip = registers[ip_reg];
        ip += 1;
    }
}
