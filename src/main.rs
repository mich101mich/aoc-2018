#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use rand::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

#[macro_use]
mod utils;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq)]
struct Army {
    immune_system: bool,
    units: usize,
    hit_points: usize,
    immunities: HashSet<&'static str>,
    weaknesses: HashSet<&'static str>,
    attack: usize,
    attack_type: &'static str,
    initiative: usize,
}
impl Army {
    pub fn effective(&self) -> usize {
        self.units * self.attack
    }
    pub fn damage(&self, other: &Army) -> usize {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            self.effective() * 2
        } else {
            self.effective()
        }
    }
}

fn to_army(s: &'static str, immune_system: bool) -> Army {
    let re = Regex::from_str(r"(?P<units>\d+) units each with (?P<hit_points>\d+) hit points (\((immune to (?P<immunities>.*?))?;? ?(weak to (?P<weaknesses>.*?))?;? ?(immune to (?P<immunities2>.*?))?\) )?with an attack that does (?P<attack>\d+) (?P<attack_type>[a-z]+) damage at initiative (?P<initiative>\d+)").expect("regex");

    let m = re.captures(s).expect("match");
    Army {
        immune_system,
        units: usize::from_str(m.name("units").expect("match units").as_str())
            .expect("parse units"),
        hit_points: usize::from_str(m.name("hit_points").expect("match hit_points").as_str())
            .expect("parse hit_points"),
        immunities: m
            .name("immunities")
            .or(m.name("immunities2"))
            .map(|m| m.as_str().split(", ").collect())
            .unwrap_or_else(HashSet::new),
        weaknesses: m
            .name("weaknesses")
            .map(|m| m.as_str().split(", ").collect())
            .unwrap_or_else(HashSet::new),
        attack: usize::from_str(m.name("attack").expect("match attack").as_str())
            .expect("parse attack"),
        attack_type: m.name("attack_type").expect("match attack_type").as_str(),
        initiative: usize::from_str(m.name("initiative").expect("match initiative").as_str())
            .expect("parse initiative"),
    }
}

fn main() {
    let input = include_str!("input/day_24.txt");

    let immune_system = input
        .lines()
        .skip(1)
        .take_while(|s| *s != "")
        .map(|s| to_army(s, true))
        .collect::<Vec<_>>();
    let infection = input
        .lines()
        .skip_while(|s| *s != "Infection:")
        .skip(1)
        .map(|s| to_army(s, false))
        .collect::<Vec<_>>();

    let mut groups = immune_system.clone();
    groups.append(&mut infection.clone());

    pv!(groups);

    'boost: for boost in 0.. {
        pv!(boost);

        let mut groups = groups.clone();
        for g in &mut groups {
            if g.immune_system {
                g.attack += boost;
            }
        }

        loop {
            groups.sort_by(|a, b| match a.effective().cmp(&b.effective()) {
                Ordering::Equal => a.initiative.cmp(&b.initiative),
                ord => ord,
            });
            groups.reverse();
            let mut matches = vec![];

            for g in 0..groups.len() {
                let current = &groups[g];
                let mut best = None;
                let mut max = 0;
                for (o, other) in groups
                    .iter()
                    .enumerate()
                    .filter(|(_, o)| o.immune_system != current.immune_system)
                    .filter(|(o, _)| matches.iter().find(|(_, target)| o == target).is_none())
                {
                    let damage = current.damage(other);
                    //println!("{} -> {}: {}", g, o, damage);
                    if damage > max {
                        max = damage;
                        best = Some(o);
                    }
                }
                if let Some(other) = best {
                    matches.push((g, other));
                }
            }

            matches.sort_by_key(|&(a, _)| -(groups[a].initiative as isize));

            let mut died = vec![];
            let mut total_defeated = 0;
            for m in 0..matches.len() {
                let (a, b) = matches[m];
                if died.contains(&a) || died.contains(&b) {
                    continue;
                }
                let damage = groups[a].damage(&groups[b]);

                let target = &mut groups[b];
                let defeated = damage / target.hit_points;

                total_defeated += defeated;

                if target.units <= defeated {
                    died.push(b);
                } else {
                    target.units -= defeated;
                }
            }

            if total_defeated == 0 {
                continue 'boost;
            }

            while let Some(dead) = died.pop() {
                groups.remove(dead);
                for other in &mut died {
                    if *other > dead {
                        *other -= 1;
                    }
                }
            }
            //pv!(groups);

            if groups.iter().find(|army| army.immune_system).is_none()
                || groups.iter().find(|army| !army.immune_system).is_none()
            {
                break;
            }
        }
        pv!(groups);
        let immune_system: usize = groups
            .iter()
            .filter(|army| army.immune_system)
            .map(|army| army.units)
            .sum();
        let infection: usize = groups
            .iter()
            .filter(|army| !army.immune_system)
            .map(|army| army.units)
            .sum();
        pv!(immune_system);
        pv!(infection);
        println!();
        if immune_system > 0 {
            break;
        }
    }
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
