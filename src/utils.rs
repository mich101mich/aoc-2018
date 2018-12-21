#![allow(unused)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

pub fn neighbours(
	(x, y): (usize, usize),
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	let mut n = vec![];
	if y > 0 {
		n.push((x, y - 1));
	}
	if x > 0 {
		n.push((x - 1, y));
	}
	if x < width - 1 {
		n.push((x + 1, y));
	}
	if y < height - 1 {
		n.push((x, y + 1));
	}
	n.into_iter()
}

pub fn manhatten(p1: (usize, usize), p2: (usize, usize)) -> usize {
	((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as usize
}

pub fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
	std::iter::repeat(std::iter::repeat(value).take(height).collect::<Vec<T>>())
		.take(width)
		.collect()
}

#[derive(Debug)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}

pub type Cost = usize;

#[derive(Debug, Clone)]
pub struct Path<P> {
	pub path: Vec<P>,
	pub cost: Cost,
}

impl<P> Path<P> {
	pub fn new(path: Vec<P>, cost: Cost) -> Path<P> {
		Path { path, cost }
	}
	pub fn append(&mut self, node: P, cost: Cost) -> &mut Self {
		self.path.push(node);
		self.cost += cost;
		self
	}
}

use std::ops::*;

impl<P> Index<usize> for Path<P> {
	type Output = P;
	fn index(&self, index: usize) -> &P {
		&self.path[index]
	}
}

pub fn ordered_insert<T, V, F>(vector: &mut Vec<T>, element: T, get_value: F)
where
	T: std::fmt::Debug,
	V: Ord,
	F: Fn(&T) -> V,
{
	let value = get_value(&element);
	for i in 0..vector.len() {
		if get_value(&vector[i]) <= value {
			vector.insert(i, element);
			return;
		}
	}
	vector.push(element);
}

pub fn a_star_search<Id, GetNeighbours, NeighbourIter, GetCost, IsWalkable, Heuristic>(
	get_all_neighbours: GetNeighbours,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goal: Id,
	heuristic: Heuristic,
) -> Option<Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + std::fmt::Debug,
	GetNeighbours: Fn(Id) -> NeighbourIter,
	NeighbourIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	Heuristic: Fn(Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	if start == goal {
		return Some(Path::new(vec![start, start], 0));
	}
	let mut visited = HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	'search: while let Some((current_id, _)) = next.pop() {
		if current_id == goal {
			break 'search;
		}
		let current_cost = visited[&current_id].0;

		for other_id in get_all_neighbours(current_id) {
			let other_cost = current_cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) && other_id != goal {
				continue;
			}

			let heuristic = heuristic(other_id);

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(
					&mut next,
					(other_id, other_cost + heuristic),
					|&(_, cost)| cost,
				);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	if !visited.contains_key(&goal) {
		return None;
	}

	let steps = {
		let mut steps = vec![];
		let mut current = goal;

		while current != start {
			steps.push(current);
			let (_, prev) = visited[&current];
			current = prev;
		}
		steps.push(start);
		steps.reverse();
		steps
	};

	Some(Path::new(steps, visited[&goal].0))
}

pub fn dijkstra_search<Id, GetNeighbours, NeighbourIter, GetCost, IsWalkable>(
	get_all_neighbours: GetNeighbours,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goals: &[Id],
) -> HashMap<Id, Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + ::std::fmt::Debug,
	GetNeighbours: Fn(Id) -> NeighbourIter,
	NeighbourIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	let mut visited = ::std::collections::HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	let mut remaining_goals = goals.to_vec();

	let mut goal_costs = HashMap::with_capacity(goals.len());

	while let Some((current_id, _)) = next.pop() {
		let cost = visited[&current_id].0;

		for &goal_id in remaining_goals.iter() {
			if current_id == goal_id {
				goal_costs.insert(goal_id, cost);
			}
		}
		remaining_goals.retain(|&id| id != current_id);
		if remaining_goals.is_empty() {
			break;
		}

		for other_id in get_all_neighbours(current_id) {
			let other_cost = cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) {
				let mut is_goal = false;
				for &goal_id in remaining_goals.iter() {
					if other_id == goal_id {
						is_goal = true;
					}
				}
				if !is_goal {
					continue;
				}
			}

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(&mut next, (other_id, other_cost), |&(_, cost)| cost);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	let mut goal_data = HashMap::with_capacity(goal_costs.len());

	for (&goal, &cost) in goal_costs.iter() {
		let steps = {
			let mut steps = vec![];
			let mut current = goal;

			while current != start {
				steps.push(current);
				let (_, prev) = visited[&current];
				current = prev;
			}
			steps.push(start);
			steps.reverse();
			steps
		};
		goal_data.insert(goal, Path::new(steps, cost));
	}

	goal_data
}

pub fn parse_asm(input: &str) -> (Vec<(&str, usize, usize, usize)>, usize) {
	let v = input
		.lines()
		.skip(1)
		.map(|line| {
			let mut sp = line.split(' ');
			let op = sp.next().unwrap();
			let n = sp
				.map(|num| usize::from_str(num).unwrap())
				.collect::<Vec<_>>();
			(op, n[0], n[1], n[2])
		})
		.collect::<Vec<_>>();
	let ip_reg = usize::from_str(&input.lines().next().unwrap()[4..]).unwrap();
	(v, ip_reg)
}

pub fn asm_run(instr: (&str, usize, usize, usize), registers: &mut [usize; 6]) {
	match instr.0 {
		"addr" => {
			let res = registers[instr.1] + registers[instr.2];
			registers[instr.3] = res;
		}
		"addi" => {
			let res = registers[instr.1] + instr.2;
			registers[instr.3] = res;
		}
		"mulr" => {
			let res = registers[instr.1] * registers[instr.2];
			registers[instr.3] = res;
		}
		"muli" => {
			let res = registers[instr.1] * instr.2;
			registers[instr.3] = res;
		}
		"banr" => {
			let res = registers[instr.1] & registers[instr.2];
			registers[instr.3] = res;
		}
		"bani" => {
			let res = registers[instr.1] & instr.2;
			registers[instr.3] = res;
		}
		"borr" => {
			let res = registers[instr.1] | registers[instr.2];
			registers[instr.3] = res;
		}
		"bori" => {
			let res = registers[instr.1] | instr.2;
			registers[instr.3] = res;
		}
		"setr" => {
			let res = registers[instr.1];
			registers[instr.3] = res;
		}
		"seti" => {
			let res = instr.1;
			registers[instr.3] = res;
		}
		"gtir" => {
			let res = instr.1 > registers[instr.2];
			registers[instr.3] = res as usize;
		}
		"gtri" => {
			let res = registers[instr.1] > instr.2;
			registers[instr.3] = res as usize;
		}
		"gtrr" => {
			let res = registers[instr.1] > registers[instr.2];
			registers[instr.3] = res as usize;
		}
		"eqir" => {
			let res = instr.1 == registers[instr.2];
			registers[instr.3] = res as usize;
		}
		"eqri" => {
			let res = registers[instr.1] == instr.2;
			registers[instr.3] = res as usize;
		}
		"eqrr" => {
			let res = registers[instr.1] == registers[instr.2];
			registers[instr.3] = res as usize;
		}
		op => panic!("no op: {}", op),
	}
}
