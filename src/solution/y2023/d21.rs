use crate::error::*;

/// # Examples
///
/// Part 1:
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
///     .##..S####.\n\
///     .##..#...#.\n\
///     .......##..\n\
///     .##.#.####.\n\
///     .##..##.##.\n\
///     ...........";
/// assert_eq!(Solution::with_steps(6, input.to_string()).solve().unwrap(), "16");
/// ```
///
/// Part 2, thanks to https://www.reddit.com/r/adventofcode/comments/18o1071/2023_day_21_a_better_example_input_mild_part_2/
///
/// ```
/// # use adventofcode::solution::{ y2023::d21::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     .................\n\
///     ..#..............\n\
///     ...##........###.\n\
///     .............##..\n\
///     ..#....#.#.......\n\
///     .......#.........\n\
///     ......##.##......\n\
///     ...##.#.....#....\n\
///     ........S........\n\
///     ....#....###.#...\n\
///     ......#..#.#.....\n\
///     .....#.#..#......\n\
///     .#...............\n\
///     .#.....#.#....#..\n\
///     ...#.........#.#.\n\
///     ...........#..#..\n\
///     .................";
/// assert_eq!(Solution::with_steps(7, input.to_string()).solve().unwrap(), "52");
/// assert_eq!(Solution::with_steps(8, input.to_string()).solve().unwrap(), "68");
/// assert_eq!(Solution::with_steps(25, input.to_string()).solve().unwrap(), "576");
/// assert_eq!(Solution::with_steps(42, input.to_string()).solve().unwrap(), "1576");
/// assert_eq!(Solution::with_steps(59, input.to_string()).solve().unwrap(), "3068");
/// assert_eq!(Solution::with_steps(76, input.to_string()).solve().unwrap(), "5052");
/// assert_eq!(Solution::with_steps(1180148, input.to_string()).solve().unwrap(), "1185525742508");
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

type Position = (usize, usize);

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
	type Output = Option<Position>;

	fn add(self, dir: Direction) -> Option<Position>
	{
		let (dy, dx) = dir.vector();
		let x = match dx
		{
			-1 => self.1.checked_sub(1),
			0 => Some(self.1),
			1 => self.1.checked_add(1),
			_ => unreachable!(),
		}?;
		let y = match dy
		{
			-1 => self.0.checked_sub(1),
			0 => Some(self.0),
			1 => self.0.checked_add(1),
			_ => unreachable!(),
		}?;
		Some((y, x))
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
						.filter_map(move |&dir| pos + dir)
						.filter(|(x, y)| (0..len).contains(x) && (0..len).contains(y))
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

		let len = self.input.lines().count();

		let garden = self.input.lines()
			.inspect(|line| trace!("parsing line {:?}", line))
			.enumerate()
			.flat_map(|(y, line)|
			{
				line.chars()
					.enumerate()
					.filter_map(move |(x, ch)|
					{
						let pos = (x, y);
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

		anyhow::ensure!(start.0 == len / 2 && start.1 == len / 2, "start must be in the middle");
		anyhow::ensure!((0..len).map(|x| (x, 0)).all(|pos| garden.contains(&pos)), "input must contain clear borders (top)");
		anyhow::ensure!((0..len).map(|x| (x, len - 1)).all(|pos| garden.contains(&pos)), "input must contain clear borders (bottom)");
		anyhow::ensure!((0..len).map(|y| (0, y)).all(|pos| garden.contains(&pos)), "input must contain clear borders (left)");
		anyhow::ensure!((0..len).map(|y| (len - 1, y)).all(|pos| garden.contains(&pos)), "input must contain clear borders (right)");

		if self.steps <= len / 2 + 1
		{
			debug!("running cheap simulation for {} steps", self.steps);
			let last = simulate(garden.clone(), start)
				.enumerate()
				.inspect(|(i, pos)| trace!("step {}: {}", i, pos.len()))
				.nth(self.steps)
				.ok_or_else(|| anyhow!("cannot get {}th step count", self.steps))?
				.1;

			return Ok(format!("{}", last.len()));
		}

		anyhow::ensure!((self.steps - len / 2) % len == 0, "part 2 optimization failed");
		anyhow::ensure!((0..len).map(|x| (x, len / 2)).all(|pos| garden.contains(&pos)), "input must contain clear horizontal line");
		anyhow::ensure!(std::iter::repeat([Direction::Left, Direction::Up]).flatten().scan((len - 1, len / 2), |state, dir| { (*state + dir).map(|res| { *state = res; res }) }).take_while(|&pos| pos != (len / 2, 0)).all(|pos| garden.contains(&pos)), "input must have clear line from right middle to top middle");
		//anyhow::ensure!(std::iter::repeat([Direction::Left, Direction::Down]).flatten().scan((len - 1, len / 2), |state, dir| { (*state + dir).map(|res| { *state = res; res }) }).take_while(|&pos| pos != (len / 2, len - 1)).all(|pos| garden.contains(&pos)), "input must have clear line from right middle to bottom middle");
		anyhow::ensure!(std::iter::repeat([Direction::Right, Direction::Up]).flatten().scan((0, len / 2), |state, dir| { (*state + dir).map(|res| { *state = res; res }) }).take_while(|&pos| pos != (len / 2, 0)).all(|pos| garden.contains(&pos)), "input must have clear line from left middle to top middle");
		anyhow::ensure!(std::iter::repeat([Direction::Right, Direction::Down]).flatten().scan((0, len / 2), |state, dir| { (*state + dir).map(|res| { *state = res; res }) }).take_while(|&pos| pos != (len / 2, len - 1)).all(|pos| garden.contains(&pos)), "input must have clear line from left middle to bottom middle");

		let even =
		{
			let n = len * 2 + self.steps % 2;
			simulate(garden.clone(), start)
				.enumerate()
				.inspect(|(i, pos)| trace!("step {}: {}", i, pos.len()))
				.nth(n)
				.ok_or_else(|| anyhow!("cannot get {}th step count", n))?
				.1
		};
		debug!("even: {}", even.len());

		let odd =
		{
			let n = len * 2 + (self.steps + 1) % 2;
			simulate(garden.clone(), start)
				.enumerate()
				.inspect(|(i, pos)| trace!("step {}: {}", i, pos.len()))
				.nth(n)
				.ok_or_else(|| anyhow!("cannot get {}th step count", n))?
				.1
		};
		debug!("odd: {}", odd.len());

		let i_diamond = (0..len)
			.flat_map(|x|
			{
				let off = if x > len / 2 { len - 1 - x } else { x };
				((len / 2 - off)..=(len / 2 + off)).map(move |y| (x, y))
			})
			.collect::<std::collections::BTreeSet<_>>();

		if true
		{
			let i_diamond = &i_diamond;
			debug!("diamond shape:\n{}", (0..len).flat_map(|y| (0..len).map(move |x| if i_diamond.contains(&(x, y)) { '#' } else { '.' }).chain(['\n'])).collect::<String>());
		}

		let large_steps = (self.steps - len / 2) / len;
		debug!("large_steps: {}", large_steps);
		let even_diamond = (large_steps | 1).pow(2);
		debug!("even_diamond: {}", even_diamond);
		let odd_diamond = (((large_steps - 1) | 1) + 1).pow(2);
		debug!("odd_diamond: {}", odd_diamond);
		let triangles = large_steps.pow(2) + large_steps;
		debug!("triangles: {}", triangles);

		let num_even_diamond = even_diamond * i_diamond.intersection(&even).count();
		debug!("num_even_diamond: {}", num_even_diamond);
		let num_odd_diamond = odd_diamond * i_diamond.intersection(&odd).count();
		debug!("num_odd_diamond: {}", num_odd_diamond);
		let num_triangles = triangles * (odd.difference(&i_diamond).count() + even.difference(&i_diamond).count());
		debug!("num_triangles: {}", num_triangles);

		let result = num_even_diamond + num_odd_diamond + num_triangles;
		debug!("result: {}", result);

		Ok(format!("{}", result))
	}
}
