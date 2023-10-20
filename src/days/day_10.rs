use crate::utils::*;
use std::ops::DerefMut;

#[derive(Debug, Clone, Copy, FromScanf)]
#[sscanf(format_unescaped = r"position=<\s*{x}, \s*{y}> velocity=<\s*{vx}, \s*{vy}>")]
struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}
impl Point {
    fn next(&self) -> Self {
        Self {
            x: self.x + self.vx,
            y: self.y + self.vy,
            ..*self
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{Point}").unwrap())
        .to_vec();

    let mut next = parsed.clone();
    let mut min_height = isize::MAX;

    for time in 0.. {
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for (cur, next) in parsed.iter().zip(next.iter_mut()) {
            *next = cur.next();
            min_y = min_y.min(next.y);
            max_y = max_y.max(next.y);
        }
        let height = max_y - min_y;
        if height <= min_height {
            min_height = height;
        } else {
            pv!(time);
            break;
        }
        std::mem::swap(&mut parsed, &mut next);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{Point}").unwrap())
        .to_vec();

    let mut next = parsed.clone();
    let mut min_height = isize::MAX;

    loop {
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for (cur, next) in parsed.iter().zip(next.iter_mut()) {
            *next = cur.next();
            min_y = min_y.min(next.y);
            max_y = max_y.max(next.y);
        }
        let height = max_y - min_y;
        if height <= min_height {
            min_height = height;
        } else {
            break;
        }
        std::mem::swap(&mut parsed, &mut next);
    }

    let (min_x, min_y, max_x) = parsed.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN),
        |(min_x, min_y, max_x), p| (min_x.min(p.x), min_y.min(p.y), max_x.max(p.x)),
    );
    let width = max_x - min_x;

    let mut display = Grid::new_clone(p2(width as usize + 1, min_height as usize + 1), false);
    for p in parsed.iter() {
        display[p2((p.x - min_x) as usize, (p.y - min_y) as usize)] = true;
    }
    display.print('#', ' ');
}
