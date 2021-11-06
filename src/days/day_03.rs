use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut cloth: Grid<usize> = Grid::new_default((1000, 1000));

    let mut possible = HashSet::new();

    input
        .lines()
        .map(|l| scanf!(l, "#{} @ {},{}: {}x{}", usize, usize, usize, usize, usize).unwrap())
        .for_each(|(id, x, y, w, h)| {
            possible.insert(id);
            for i in x..x + w {
                for j in y..y + h {
                    let cell = &mut cloth[(i, j)];
                    if *cell != 0 {
                        possible.remove(&id);
                        possible.remove(cell);
                    } else {
                        *cell = id;
                    }
                }
            }
        });
    pv!(possible);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut cloth: Grid<u8> = Grid::new_default((1000, 1000));

    input
        .lines()
        .map(|l| scanf!(l, "#{} @ {},{}: {}x{}", usize, usize, usize, usize, usize).unwrap())
        .for_each(|(id, x, y, w, h)| {
            for i in x..x + w {
                for j in y..y + h {
                    cloth[(i, j)] += 1;
                }
            }
        });

    let count = cloth.grid_iter().filter(|&v| *v > 1).count();
    pv!(count);
}
