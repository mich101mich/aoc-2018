use crate::utils::*;

fn visit(
    src: &mut std::str::Chars,
    positions: &mut Vec<Point>,
    possible: &mut HashMap<Point, [bool; 4]>,
) {
    let backup = positions.clone();
    let mut alternatives = vec![];
    while let Some(c) = src.next() {
        match c {
            '(' => {
                visit(src, positions, possible);
            }
            '|' => {
                alternatives.push(std::mem::replace(positions, backup.clone()));
            }
            ')' => {
                break;
            }
            'N' | 'E' | 'S' | 'W' => {
                let dir = Dir::from(c);
                let num = dir.num();
                let opp = dir.opposite().num();
                for pos in positions.iter_mut() {
                    possible.entry(*pos).or_insert([false; 4])[num] = true;
                    *pos += dir;
                    possible.entry(*pos).or_insert([false; 4])[opp] = true;
                }
            }
            c => {
                panic!("Unexpected character: {}", c);
            }
        }
    }
    if !alternatives.is_empty() {
        for mut alt in alternatives {
            positions.append(&mut alt);
        }
        positions.sort_unstable();
        positions.dedup();
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let parsed = input.strip_prefix('^').unwrap().strip_suffix('$').unwrap();

    let mut possible = HashMap::new();
    visit(&mut parsed.chars(), &mut vec![(0, 0)], &mut possible);

    let all_rooms = possible.keys().copied().to_vec();
    let paths = dijkstra_search(
        |pos, out| {
            for (d, p) in possible[&pos].iter().enumerate() {
                if *p {
                    out.push(pos + Dir::from(d));
                }
            }
        },
        (0, 0),
        &all_rooms,
    );
    let count = paths.values().filter(|p| p.cost >= 1000).count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let parsed = input.strip_prefix('^').unwrap().strip_suffix('$').unwrap();

    let mut possible = HashMap::new();
    visit(&mut parsed.chars(), &mut vec![(0, 0)], &mut possible);

    let all_rooms = possible.keys().copied().to_vec();
    let paths = dijkstra_search(
        |pos, out| {
            for (d, p) in possible[&pos].iter().enumerate() {
                if *p {
                    out.push(pos + Dir::from(d));
                }
            }
        },
        (0, 0),
        &all_rooms,
    );
    let longest = paths.values().map(|p| p.cost).max().unwrap();
    pv!(longest);
}
