use crate::utils::*;

fn power_level(serial: isize, x: isize, y: isize) -> isize {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let serial = parse(input);

    let mut grid = Grid::new_clone(p2(300, 300), 0isize);
    let mut integral_image = Grid::new_clone(p2(300, 300), 0isize);

    for x in 0..300 {
        let mut row = 0;
        for y in 0..300 {
            let power = power_level(serial, x + 1, y + 1);
            grid[p2(x, y)] = power;
            row += power;
            integral_image[p2(x, y)] = row
                + if x > 0 {
                    integral_image[p2(x - 1, y)]
                } else {
                    0
                };
        }
    }

    let mut max_power = isize::MIN;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;

    for size in 1..=300isize {
        for x in 0..=300 - size {
            for y in 0..=300 - size {
                let mut power = integral_image[p2(x + size - 1, y + size - 1)];
                if x > 0 {
                    power -= integral_image[p2(x - 1, y + size - 1)];
                    if y > 0 {
                        power += integral_image[p2(x - 1, y - 1)]
                    }
                }
                if y > 0 {
                    power -= integral_image[p2(x + size - 1, y - 1)]
                }

                if power > max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                    max_size = size;
                }
            }
        }
    }

    println!("{},{},{}", max_x + 1, max_y + 1, max_size);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let serial = parse(input);

    let mut grid = Grid::new_clone(p2(300, 300), 0isize);

    for x in 0..300 {
        for y in 0..300 {
            let power = power_level(serial, x + 1, y + 1);
            for px in (x - 2).max(0)..=x {
                for py in (y - 2).max(0)..=y {
                    grid[p2(px, py)] += power;
                }
            }
        }
    }

    let (x, y) = grid
        .grid_iter_index()
        .filter(|(p, _)| p.x < 297 && p.y < 297)
        .max_by_key(|(_, v)| *v)
        .map(|(p, _)| (p.x + 1, p.y + 1))
        .unwrap();

    println!("{},{}", x, y);
}
