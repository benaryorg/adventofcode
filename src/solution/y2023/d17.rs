use crate::error::*;

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d17::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     2413432311323\n\
///     3215453535623\n\
///     3255245654254\n\
///     3446585845452\n\
///     4546657867536\n\
///     1438598798454\n\
///     4457876987766\n\
///     3637877979653\n\
///     4654967986887\n\
///     4564679986453\n\
///     1224686865563\n\
///     2546548887735\n\
///     4322674655533";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "102");
/// ```
///
/// Part 2:
///
/// Does not actually work at this time.
///
/// ```no_run
/// # use adventofcode::solution::{ y2023::d17::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     2413432311323\n\
///     3215453535623\n\
///     3255245654254\n\
///     3446585845452\n\
///     4546657867536\n\
///     1438598798454\n\
///     4457876987766\n\
///     3637877979653\n\
///     4654967986887\n\
///     4564679986453\n\
///     1224686865563\n\
///     2546548887735\n\
///     4322674655533";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "94");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self { part: Part::One, input, }
	}

	pub fn part2(input: String) -> Self
	{
		Self { part: Part::Two, input, }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Part
{
	One,
	Two,
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
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
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

	fn symbol(&self) -> char
	{
		match &self
		{
			Direction::Up => '^',
			Direction::Down => 'v',
			Direction::Left => '<',
			Direction::Right => '>',
		}
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
		let (dx, dy) = dir.vector();
		let x = match dx
		{
			-1 => self.0.checked_sub(1),
			0 => Some(self.0),
			1 => self.0.checked_add(1),
			_ => unreachable!(),
		}?;
		let y = match dy
		{
			-1 => self.1.checked_sub(1),
			0 => Some(self.1),
			1 => self.1.checked_add(1),
			_ => unreachable!(),
		}?;
		Some((x, y))
	}
}

fn calculate(field: &[&[usize]], min_turn: usize, max_turn: usize) -> (usize, Vec<Direction>)
{
	let mut memo = std::collections::BTreeMap::<(Position, (Direction, usize)), (usize, Vec<Direction>)>::new();

	memo.extend(Direction::slice().iter().flat_map(|&dir| (min_turn..max_turn).map(move |count| (dir, count))).map(|(dir, count)| (((field.len() - 1, field.len() - 1), (dir, count)), (0, Vec::new()))));

	loop
	{
		let mut changed = false;
		for pos in (0..field.len()).rev().flat_map(move |x| (0..field.len()).rev().map(move |y| (x, y)))
		{
			for &idir in Direction::slice().iter()
			{
				for odir in [idir, idir.left(), idir.right()]
				{
					for icounter in min_turn..=max_turn
					{
						if let Some((npos, val)) = (pos + odir)
							.and_then(|(x, y)|
							{
								let row = field.get(y)?;
								let val = row.get(x)?;
								Some(((x, y), val))
							})
						{
							let bound = if idir == odir
							{
								if icounter == 0
								{
									continue;
								}
								else
								{
									icounter - 1
								}
							}
							else
							{
								if icounter >= max_turn - min_turn
								{
									continue;
								}
								else
								{
									max_turn - 1
								}
							};
							let entry = if let Some((dist, dirs)) = memo.get(&(npos, (odir, icounter)))
							{
								let dirs = dirs.iter().copied().chain([odir]).collect();
								Some((*dist + val, dirs))
							}
							else
							{
								None
							};
							if let Some(entry) = entry
							{
								memo.entry((pos, (idir, bound)))
									.and_modify(|data|
									{
										if entry.0 < data.0
										{
											changed = true;
											*data = entry.clone();
										}
									})
									.or_insert_with(||
									{
										
										changed = true;
										entry
									});
							}
						}
					}
				}
			}
		}
		if !changed
		{
			debug!("no more changes (len: {})", memo.len());
			break;
		}
		debug!("has changed (len: {})", memo.len());
	}

	Direction::slice().iter()
		.flat_map(|&dir| ((min_turn+1)..=max_turn).map(move |count| (dir, count)))
		.filter_map(|(dir, count)| memo.get(&((0, 0), (dir, count))))
		.min_by_key(|(dist, _)| dist)
		.cloned()
		.unwrap()
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let field = self.input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| ch.to_digit(10).map(|i| i as usize).ok_or_else(|| anyhow!("char {} not a digit", ch)))
					.collect::<Result<Vec<usize>>>()
					.context(Error::AocParsing)
			})
			.collect::<Result<Vec<Vec<usize>>>>()?;

		debug!("field:\n{:?}", field);

		let field: &[&[usize]] = &field.iter()
			.map(|row| row.as_slice())
			.collect::<Vec<_>>();

		let path = match self.part
		{
			Part::One => calculate(field, 0, 3),
			Part::Two => calculate(field, 5, 10),
		};

		debug!("taking path with length {}: {:?}", path.0, path.1);
		let mut sum = 0;
		let steps = path.1.iter()
			.rev()
			.copied()
			.scan((0, 0), |pos, dir|
			{
				let npos = (*pos + dir).unwrap();
				let val = field[npos.1][npos.0];
				sum += val;
				trace!("step from {:?} {:?} to {:?}: +{} -> sum: {}", pos, dir, npos, val, sum);
				*pos = npos;
				Some((npos, dir))
			})
			.collect::<std::collections::BTreeMap<Position, Direction>>();
		let steps = &steps;
		let view = field.iter()
			.enumerate()
			.flat_map(|(y, row)|
			{
				row.iter()
					.enumerate()
					.map(move |(x, &val)|
					{
						steps.get(&(x, y))
							.map(Direction::symbol)
							.unwrap_or_else(|| char::from_digit(val as u32, 10).unwrap())
					})
					.chain(['\n'])
			})
			.collect::<String>();
		debug!("view:\n{}", view);

		Ok(format!("{}", path.0))
	}
}

