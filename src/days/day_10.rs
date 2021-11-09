use crate::utils::*;
use std::ops::DerefMut;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input
        .lines()
        .map(|l| {
            scanf_unescaped!(
                l,
                r"position=<\s*{}, \s*{}> velocity=<\s*{}, \s*{}>",
                isize,
                isize,
                isize,
                isize
            )
            .unwrap()
        })
        .to_vec();

    let mut next = parsed.clone();
    let mut min_height = isize::MAX;

    for time in 0.. {
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for ((x, y, vx, vy), (tx, ty, ..)) in parsed.iter().zip(next.iter_mut()) {
            *tx = *x + *vx;
            *ty = *y + *vy;
            min_y = min_y.min(*ty);
            max_y = max_y.max(*ty);
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
        .map(|l| {
            scanf_unescaped!(
                l,
                r"position=<\s*{}, \s*{}> velocity=<\s*{}, \s*{}>",
                isize,
                isize,
                isize,
                isize
            )
            .unwrap()
        })
        .to_vec();

    let mut next = parsed.clone();
    let mut min_height = isize::MAX;

    loop {
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for ((x, y, vx, vy), (tx, ty, ..)) in parsed.iter().zip(next.iter_mut()) {
            *tx = *x + *vx;
            *ty = *y + *vy;
            min_y = min_y.min(*ty);
            max_y = max_y.max(*ty);
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
        |(min_x, min_y, max_x), (x, y, ..)| (min_x.min(*x), min_y.min(*y), max_x.max(*x)),
    );
    let width = max_x - min_x;

    let mut display = Grid::new_clone((width as usize + 1, min_height as usize + 1), false);
    for (x, y, ..) in parsed.iter() {
        display[((x - min_x) as usize, (y - min_y) as usize)] = true;
    }
    display.print('#', ' ');
}
