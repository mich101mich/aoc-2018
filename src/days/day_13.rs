use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");
    //     let input = "/>-<\\
    // |   |
    // | /<+-\\
    // | | | v
    // \\>+</ |
    //   |   ^
    //   \\<->/";

    let mut grid = char_grid(input);
    let mut carts = grid
        .grid_iter_index()
        .filter(|(_, c)| matches!(c, '^' | 'v' | '<' | '>'))
        .map(|(p, c)| (p, Dir::from_str(&c.to_string()).unwrap(), 0))
        .to_vec();

    for (p, d, _) in &carts {
        if d.is_vertical() {
            grid[*p] = '|';
        } else {
            grid[*p] = '-';
        }
    }

    let mut positions = HashSet::new();
    let mut remove = HashSet::new();
    for tick in 0.. {
        positions.clear();
        remove.clear();
        carts.sort_by(|(p1, _, _), (p2, _, _)| p1.y.cmp(&p2.y).then(p1.x.cmp(&p2.x)));
        for (p, d, turn) in &mut carts {
            if remove.contains(p) {
                continue;
            }
            if positions.contains(p) {
                remove.insert(*p);
                continue;
            }
            *p += *d;
            if !positions.insert(*p) {
                remove.insert(*p);
                continue;
            }

            match (grid[*p], d.is_vertical()) {
                ('|', true) => (),
                ('-', false) => (),
                ('+', _) => {
                    *d = match *turn {
                        0 => d.counter_clockwise(),
                        1 => *d,
                        2 => d.clockwise(),
                        x => panic!("Invalid turn: {}", x),
                    };
                    *turn = (*turn + 1) % 3
                }
                ('/', true) | ('\\', false) => *d = d.clockwise(),
                ('/', false) | ('\\', true) => *d = d.counter_clockwise(),
                x => panic!("Unexpected situation: {:?} {:?}", x, *d),
            }
        }
        carts.retain(|(p, _, _)| !remove.contains(p));
        if carts.len() == 1 {
            let p = carts[0].0;
            println!("{},{}", p.x, p.y);
            break;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut grid = char_grid(input);
    let mut carts = grid
        .grid_iter_index()
        .filter(|(_, c)| matches!(c, '^' | 'v' | '<' | '>'))
        .map(|(p, c)| (p, Dir::from_str(&c.to_string()).unwrap(), 0))
        .to_vec();

    for (p, d, _) in &carts {
        if d.is_vertical() {
            grid[*p] = '|';
        } else {
            grid[*p] = '-';
        }
    }

    loop {
        let mut positions = HashSet::new();
        for (p, d, turn) in &mut carts {
            *p += *d;
            if !positions.insert(*p) {
                println!("{},{}", p.x, p.y);
                return;
            }

            match (grid[*p], d.is_vertical()) {
                ('|', true) => (),
                ('-', false) => (),
                ('+', _) => {
                    *d = match *turn {
                        0 => d.counter_clockwise(),
                        1 => *d,
                        2 => d.clockwise(),
                        x => panic!("Invalid turn: {}", x),
                    };
                    *turn = (*turn + 1) % 3
                }
                ('/', true) | ('\\', false) => *d = d.clockwise(),
                ('/', false) | ('\\', true) => *d = d.counter_clockwise(),
                x => panic!("Unexpected situation: {:?} {:?}", x, *d),
            }
        }
    }
}
