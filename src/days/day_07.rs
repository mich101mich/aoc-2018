use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let parsed = input.lines().map(|l| {
        scanf!(
            l,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        )
        .unwrap()
    });

    let mut pre = ('A'..='Z').map(|c| (c, vec![])).collect::<Vec<_>>();

    for (condition, result) in parsed {
        pre[(result as u8 - b'A') as usize].1.push(condition);
    }

    let mut time = 0;
    let mut busy_until = [0; 5];
    let mut completed_at = HashMap::new();

    while !pre.is_empty() {
        let mut available = pre
            .iter()
            .filter(|(_, prerequisites)| {
                prerequisites
                    .iter()
                    .all(|c| completed_at.get(c).filter(|t| **t <= time).is_some())
            })
            .map(|(c, _)| *c)
            .to_vec();

        available.sort_by(|a, b| a.cmp(b).reverse());

        for t in busy_until.iter_mut().filter(|t| **t <= time) {
            if let Some(next) = available.pop() {
                let completion = time + 60 + (next as u8 - b'A' + 1) as usize;
                completed_at.insert(next, completion);
                *t = completion;
                pre.retain(|(c, _)| *c != next);
            }
        }

        time = *busy_until.iter().filter(|t| **t > time).min().unwrap();
    }
    let max = completed_at.values().max().unwrap();
    pv!(max);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let parsed = input.lines().map(|l| {
        scanf!(
            l,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        )
        .unwrap()
    });

    let mut pre = ('A'..='Z').map(|c| (c, HashSet::new())).collect::<Vec<_>>();

    for (condition, result) in parsed {
        pre[(result as u8 - b'A') as usize].1.insert(condition);
    }

    while !pre.is_empty() {
        let i = pre
            .iter()
            .filter(|(_, set)| set.is_empty())
            .map(|(c, _)| *c)
            .min()
            .unwrap();

        pre.retain(|(c, _)| *c != i);
        pre.iter_mut().for_each(|(_, set)| {
            set.remove(&i);
        });

        print!("{}", i);
    }
}
