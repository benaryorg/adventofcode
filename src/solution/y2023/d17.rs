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
/// assert_eq!(Solution::with_min_max(1, 3, input.to_string()).solve().unwrap(), "102");
/// ```
///
/// Part 2:
///
/// Does not actually work at this time.
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
/// assert_eq!(Solution::with_min_max(4, 10, input.to_string()).solve().unwrap(), "94");
/// ```
pub struct Solution
{
	input: String,
	min: usize,
	max: usize,
}

impl Solution
{
	pub fn with_min_max(min: usize, max: usize, input: String) -> Self
	{
		Self { min, max, input, }
	}
}

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Orientation
{
	Horizontal,
	Vertical,
}

impl Orientation
{
	fn invert(&self) -> Orientation
	{
		match &self
		{
			Orientation::Horizontal => Orientation::Vertical,
			Orientation::Vertical => Orientation::Horizontal,
		}
	}

	fn slice() -> &'static [Orientation]
	{
		&[
			Orientation::Vertical,
			Orientation::Horizontal,
		]
	}
}

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

	fn orientation(&self) -> Orientation
	{
		match &self
		{
			Direction::Up | Direction::Down => Orientation::Vertical,
			Direction::Left | Direction::Right => Orientation::Horizontal,
		}
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
	let mut memo = std::collections::BTreeMap::<(Position, Orientation), (usize, Vec<Direction>)>::new();

	memo.extend(Orientation::slice().iter().map(|&o| (((field.len() - 1, field.len() - 1), o), (0, Vec::new()))));

	loop
	{
		let mut changed = false;
		for pos in (0..field.len()).rev().flat_map(move |x| (0..field.len()).rev().map(move |y| (x, y)))
		{
			for &odir in Direction::slice().iter()
			{
				'distances: for distance in min_turn..=max_turn
				{
					trace!("going from {:?} {:?} for {} steps", pos, odir, distance);
					let mut npos = pos;
					let mut cost = 0;
					for _ in 0..distance
					{
						if let Some((p, val)) = (npos + odir)
							.and_then(|(x, y)|
							{
								let row = field.get(y)?;
								let val = row.get(x)?;
								Some(((x, y), val))
							})
						{
							npos = p;
							cost += val;
						}
						else
						{
							continue 'distances;
						}
					}
					let entry = if let Some((dist, dirs)) = memo.get(&(npos, odir.orientation().invert()))
					{
						let dirs = dirs.iter().copied().chain(std::iter::repeat(odir).take(distance)).collect::<Vec<_>>();
						Some((*dist + cost, dirs))
					}
					else
					{
						None
					};
					trace!("going from {:?} {:?} for {} steps to {:?} for {} at {:?}", pos, odir, distance, npos, cost, entry);
					if let Some(entry) = entry
					{
						memo.entry((pos, odir.orientation()))
							.and_modify(|data|
							{
								if entry.0 < data.0
								{
									debug!("changing {:?} {:?} to {:?} for {} in {} steps", pos, odir, npos, cost, entry.1.len());
									changed = true;
									*data = entry.clone();
								}
							})
							.or_insert_with(||
							{
								debug!("creating {:?} {:?} to {:?} for {} in {} steps", pos, odir, npos, cost, entry.1.len());
								changed = true;
								entry
							});
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

	Orientation::slice().iter()
		.filter_map(|&orientation| memo.get(&((0, 0), orientation)))
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

		let path = calculate(field, self.min, self.max);
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

