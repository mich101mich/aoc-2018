use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let parsed = input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (c as u8 - b'a' + 1) as i8
            } else {
                -((c as u8 - b'A' + 1) as i8)
            }
        })
        .to_vec();

    let min = ('a'..='z')
        .map(|c| (c as u8 - b'a' + 1) as i8)
        .map(|c| {
            let modified = parsed
                .iter()
                .filter(|x| **x != c && **x != -c)
                .copied()
                .to_vec();
            collapse(modified)
        })
        .min()
        .unwrap();
    pv!(min);

    fn collapse(mut polymer: Vec<i8>) -> usize {
        let mut stack = Vec::with_capacity(polymer.len());
        while let [.., a, b] = polymer.as_slice() {
            if *a == -*b {
                polymer.pop();
                polymer.pop();
                if let Some(r) = stack.pop() {
                    polymer.push(r);
                }
            } else {
                stack.push(*b);
                polymer.pop();
            }
        }
        polymer.len() + stack.len()
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut parsed = input.chars().to_vec();
    let mut i = 0;
    while i < parsed.len() - 1 {
        let c = parsed[i];
        let matching = if c.is_ascii_lowercase() {
            c.to_ascii_uppercase() == parsed[i + 1]
        } else {
            c.to_ascii_lowercase() == parsed[i + 1]
        };
        if matching {
            parsed.remove(i);
            parsed.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    pv!(parsed.len());
}
