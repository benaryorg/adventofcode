use crate::error::*;

/// # Examples
///
/// Note: this is not the upstream examples, for performance reasons.
///
/// ```
/// # use adventofcode::solution::{ y2023::d21::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     ...........\n\
///     .....###.#.\n\
///     .###.##..#.\n\
///     ..#.#...#..\n\
///     ....#.#....\n\
///     .....S.....\n\
///     .##..#...#.\n\
///     .......##..\n\
///     .##.#.####.\n\
///     .##..##.##.\n\
///     ...........";
/// assert_eq!(Solution::with_steps(6, input.to_string()).solve().unwrap(), "26");
/// assert_eq!(Solution::with_steps(10, input.to_string()).solve().unwrap(), "67");
/// assert_eq!(Solution::with_steps(50, input.to_string()).solve().unwrap(), "1815");
/// assert_eq!(Solution::with_steps(100, input.to_string()).solve().unwrap(), "7246");
/// assert_eq!(Solution::with_steps(500, input.to_string()).solve().unwrap(), "180349");
/// //assert_eq!(Solution::with_steps(1000, input.to_string()).solve().unwrap(), "668697");
/// //assert_eq!(Solution::with_steps(5000, input.to_string()).solve().unwrap(), "16733044");
/// ```
pub struct Solution
{
	input: String,
	steps: usize,
}

impl Solution
{
	pub fn with_steps(steps: usize, input: String) -> Self
	{
		Self { steps, input, }
	}
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Direction
{
	Up,
	Down,
	Left,
	Right,
}

#[allow(unused)]
impl Direction
{
	fn vector(&self) -> (isize, isize)
	{
		match &self
		{
			Direction::Up => (-1, 0),
			Direction::Down => (1, 0),
			Direction::Left => (0, -1),
			Direction::Right => (0, 1),
		}
	}

	fn slice() -> &'static [Direction]
	{
		&[
			Direction::Right,
			Direction::Down,
			Direction::Left,
			Direction::Up,
		]
	}

	fn left(&self) -> Direction
	{
		Direction::slice().iter().rev().cycle().skip_while(|&dir| dir != self).nth(1).copied().unwrap()
	}

	fn right(&self) -> Direction
	{
		Direction::slice().iter().cycle().skip_while(|&dir| dir != self).nth(1).copied().unwrap()
	}
}

impl std::default::Default for Direction
{
	fn default() -> Self
	{
		Direction::slice().first().copied().unwrap()
	}
}

impl std::ops::Add<Direction> for Position
{
	type Output = Position;

	fn add(self, dir: Direction) -> Position
	{
		let (dy, dx) = dir.vector();
		let x = self.0 + dx;
		let y = self.1 + dy;
		(x, y)
	}
}

fn simulate(garden: std::collections::BTreeSet<Position>, start: Position) -> Box<impl Iterator<Item=std::collections::BTreeSet<Position>>>
{
	let len = garden.last().unwrap().0 + 1;
	Box::new(std::iter::successors(Some(std::collections::BTreeSet::from([start])), move |prev|
		{
			Some(prev.iter()
				.copied()
				.flat_map(|pos|
				{
					Direction::slice()
						.iter()
						.map(move |&dir| pos + dir)
						.inspect(move |npos| trace!("going from {:?} to {:?}", pos, npos))
						.filter(|(x, y)|
						{
							let x = x.rem_euclid(len);
							let y = y.rem_euclid(len);
							garden.contains(&(x, y))
						})
				})
				.collect())
		}))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input:\n{}", self.input);

		let start = std::cell::Cell::new(None);
		let start = &start;

		let len = self.input.lines().count() as isize;

		let garden = self.input.lines()
			.inspect(|line| trace!("parsing line {:?}", line))
			.enumerate()
			.flat_map(|(y, line)|
			{
				line.chars()
					.enumerate()
					.filter_map(move |(x, ch)|
					{
						let pos = (x as isize, y as isize);
						match ch
						{
							'.' => Some(pos),
							'S' =>
							{
								start.set(Some(pos));
								Some(pos)
							},
							_ => None,
						}
					})
			})
			.collect::<std::collections::BTreeSet<Position>>();

		let start = start.get().ok_or_else(|| anyhow!("no start was found")).context(Error::AocParsing)?;
		debug!("starting at {:?}", start);

		anyhow::ensure!((0..len).map(|x| (x, start.1)).all(|pos| garden.contains(&pos)), "input must contain clear horizontal line");
		anyhow::ensure!((0..len).map(|x| (x, 0)).all(|pos| garden.contains(&pos)), "input must contain clear borders (top)");
		anyhow::ensure!((0..len).map(|x| (x, len - 1)).all(|pos| garden.contains(&pos)), "input must contain clear borders (bottom)");
		anyhow::ensure!((0..len).map(|y| (0, y)).all(|pos| garden.contains(&pos)), "input must contain clear borders (left)");
		anyhow::ensure!((0..len).map(|y| (len - 1, y)).all(|pos| garden.contains(&pos)), "input must contain clear borders (right)");

		let last = simulate(garden.clone(), start)
			.enumerate()
			.inspect(|(i, pos)| trace!("step {}: {}", i, pos.len()))
			.nth(self.steps)
			.ok_or_else(|| anyhow!("cannot get {}th step count", self.steps))?
			.1;

		debug!("first: {:?}", last.iter().map(|&(x, y)| (x - start.0, y - start.1)).take(5).collect::<Vec<_>>());
		debug!("last: {:?}", last.iter().rev().map(|&(x, y)| (x - start.0, y - start.1)).take(5).collect::<Vec<_>>());

		let result: usize = last.len();

		Ok(format!("{}", result))
	}
}
