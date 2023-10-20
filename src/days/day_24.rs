use crate::utils::*;

#[derive(Debug, Clone)]
struct Specials {
    immunities: HashSet<String>,
    weaknesses: HashSet<String>,
}
impl RegexRepresentation for Specials {
    const REGEX: &'static str = r".*?";
}
impl FromStr for Specials {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Specials {
            immunities: HashSet::new(),
            weaknesses: HashSet::new(),
        };
        if s.is_empty() {
            return Ok(ret);
        }
        let s = s.strip_prefix('(').unwrap().strip_suffix(") ").unwrap();
        for s in s.split("; ") {
            if let Some(s) = s.strip_prefix("immune to ") {
                ret.immunities.extend(s.split(", ").map(|s| s.to_string()));
            } else if let Some(s) = s.strip_prefix("weak to ") {
                ret.weaknesses.extend(s.split(", ").map(|s| s.to_string()));
            } else {
                panic!("unexpected specials: {}", s);
            }
        }
        Ok(ret)
    }
}

#[derive(Debug, Clone, FromScanf)]
#[sscanf(
    format = "{count} units each with {hp} hit points {specials}with an attack that does {attack} {attack_type} damage at initiative {initiative}"
)]
struct Unit {
    count: usize,
    hp: usize,
    attack: usize,
    attack_type: String,
    initiative: usize,
    specials: Specials,
}

impl Unit {
    pub fn effective_power(&self) -> usize {
        self.count * self.attack
    }
    pub fn weak_to(&self, attack_type: &str) -> bool {
        self.specials.weaknesses.contains(attack_type)
    }
    pub fn immune_to(&self, attack_type: &str) -> bool {
        self.specials.immunities.contains(attack_type)
    }
}

fn simulate(mut immune: Vec<Unit>, mut infection: Vec<Unit>) -> (usize, usize) {
    while !immune.is_empty() && !infection.is_empty() {
        let mut all_units = (0..immune.len())
            .map(|i| (i, true, &immune[i]))
            .chain((0..infection.len()).map(|i| (i, false, &infection[i])))
            .to_vec();
        all_units.sort_unstable_by_key(|&(_, _, u)| {
            std::cmp::Reverse((u.effective_power(), u.initiative))
        });

        let mut selected = HashSet::new();
        let mut targets = vec![];
        for &(i, is_immune, unit) in &all_units {
            let power = unit.effective_power();

            let target = all_units
                .iter()
                .filter(|(j, b, _)| *b != is_immune && !selected.contains(&(*j, *b)))
                .filter(|(_, _, u)| !u.immune_to(&unit.attack_type))
                .map(|(j, _, u)| {
                    let mut damage = power;
                    if u.weak_to(&unit.attack_type) {
                        damage *= 2;
                    }
                    (j, damage, u.effective_power(), u.initiative)
                })
                .max_by_key(|&(_, dmg, pow, ini)| (dmg, pow, ini));
            if let Some((j, ..)) = target {
                selected.insert((*j, !is_immune));
                targets.push(((i, is_immune), *j));
            }
        }

        if targets.is_empty() {
            return (0, 0);
        }

        targets.sort_unstable_by_key(|&((i, b), _)| {
            std::cmp::Reverse(if b { &immune } else { &infection }[i].initiative)
        });

        let mut total_killed = 0;
        for ((i, is_immune), j) in targets {
            let (unit, target) = if is_immune {
                (&immune[i], &mut infection[j])
            } else {
                (&infection[i], &mut immune[j])
            };
            let mut damage = unit.effective_power();
            if target.weak_to(&unit.attack_type) {
                damage *= 2;
            }
            let defeated = (damage / target.hp).min(target.count);
            total_killed += defeated;
            target.count -= defeated;
        }
        if total_killed == 0 {
            return (0, 0);
        }

        immune.retain(|u| u.count > 0);
        infection.retain(|u| u.count > 0);
    }
    (
        immune.iter().map(|u| u.count).sum(),
        infection.iter().map(|u| u.count).sum(),
    )
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut iter = input.lines();
    iter.next().unwrap(); // intro text
    let base_immune = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{Unit}").unwrap())
        .to_vec();

    iter.next().unwrap(); // intro text
    let base_infection = iter.map(|l| sscanf!(l, "{Unit}").unwrap()).to_vec();

    let boost = binary_search(0, |boost| {
        let mut immune = base_immune.clone();
        let mut infection = base_infection.clone();
        immune.iter_mut().for_each(|u| u.attack += boost);

        let (immune_count, _) = simulate(immune, infection);

        immune_count > 0
    });

    let mut immune = base_immune.clone();
    let mut infection = base_infection;
    immune.iter_mut().for_each(|u| u.attack += boost);

    let (immune_count, _) = simulate(immune, infection);

    pv!(immune_count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut iter = input.lines();
    iter.next().unwrap(); // intro text
    let mut immune = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{Unit}").unwrap())
        .to_vec();

    iter.next().unwrap(); // intro text
    let mut infection = iter.map(|l| sscanf!(l, "{Unit}").unwrap()).to_vec();

    let (a, b) = simulate(immune, infection);

    let total_units = a + b;
    pv!(total_units);
}
