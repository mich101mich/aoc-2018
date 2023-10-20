use crate::utils::*;

#[derive(FromScanf)]
#[sscanf(format = "#{id} @ {x},{y}: {width}x{height}")]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut cloth: Grid<usize> = Grid::new_default(p2(1000, 1000));

    let mut possible = HashSet::new();

    input
        .lines()
        .map(|l| sscanf!(l, "{Claim}").unwrap())
        .for_each(|claim| {
            possible.insert(claim.id);
            for i in claim.x..claim.x + claim.width {
                for j in claim.y..claim.y + claim.height {
                    let cell = &mut cloth[p2(i, j)];
                    if *cell != 0 {
                        possible.remove(&claim.id);
                        possible.remove(cell);
                    } else {
                        *cell = claim.id;
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

    let mut cloth: Grid<u8> = Grid::new_default(p2(1000, 1000));

    input
        .lines()
        .map(|l| sscanf!(l, "{Claim}").unwrap())
        .for_each(|claim| {
            for i in claim.x..claim.x + claim.width {
                for j in claim.y..claim.y + claim.height {
                    cloth[p2(i, j)] += 1;
                }
            }
        });

    let count = cloth.grid_iter().filter(|&v| *v > 1).count();
    pv!(count);
}
