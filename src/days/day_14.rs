use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let parsed = parse_u(input);
    let mut pattern = parsed.to_string().chars().map(parse_c).to_vec();

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    let mut recipes = vec![3, 7];

    let mut state = 0;
    fn advance_sm(state: &mut usize, next: usize, pattern: &[usize]) -> bool {
        if next == pattern[*state] {
            *state += 1;
        } else {
            *state = 0;
        }
        *state == pattern.len()
    }

    loop {
        let mut result = recipes[elf_1] + recipes[elf_2];
        if result >= 10 {
            recipes.push(result / 10);
            if advance_sm(&mut state, result / 10, &pattern) {
                break;
            }
        }
        recipes.push(result % 10);
        if advance_sm(&mut state, result % 10, &pattern) {
            break;
        }
        elf_1 = (elf_1 + 1 + recipes[elf_1]) % recipes.len();
        elf_2 = (elf_2 + 1 + recipes[elf_2]) % recipes.len();
    }
    pv!(recipes.len() - pattern.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let parsed = parse_u(input);

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    let mut recipes = vec![3, 7];

    while recipes.len() < parsed + 10 {
        let mut result = recipes[elf_1] + recipes[elf_2];
        if result >= 10 {
            recipes.push(result / 10);
        }
        recipes.push(result % 10);
        elf_1 = (elf_1 + 1 + recipes[elf_1]) % recipes.len();
        elf_2 = (elf_2 + 1 + recipes[elf_2]) % recipes.len();
    }
    for n in &recipes[parsed..parsed + 10] {
        print!("{}", n);
    }
    println!();
}
