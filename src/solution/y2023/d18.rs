use crate::error::*;

use std::ops::Bound;

/// Whatever you do, do not run this in production.
/// The "cutout" search algorithm eats memory like no tomorrow (although it *is* fast) and the entire thing only works because I compress the 2D shape down to the essentials (no 10000 long trenches, only the minimum required to retain the shape.
///
/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d18::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     R 6 (#70c710)\n\
///     D 5 (#0dc571)\n\
///     L 2 (#5713f0)\n\
///     D 2 (#d2c081)\n\
///     R 2 (#59c680)\n\
///     D 2 (#411b91)\n\
///     L 5 (#8ceee2)\n\
///     U 2 (#caa173)\n\
///     L 1 (#1b58a2)\n\
///     U 2 (#caa171)\n\
///     R 2 (#7807d2)\n\
///     U 3 (#a77fa3)\n\
///     L 2 (#015232)\n\
///     U 2 (#7a21e3)";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "62");
/// ```
///
/// Part 2:
///
/// ```
/// # use adventofcode::solution::{ y2023::d18::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     R 6 (#70c710)\n\
///     D 5 (#0dc571)\n\
///     L 2 (#5713f0)\n\
///     D 2 (#d2c081)\n\
///     R 2 (#59c680)\n\
///     D 2 (#411b91)\n\
///     L 5 (#8ceee2)\n\
///     U 2 (#caa173)\n\
///     L 1 (#1b58a2)\n\
///     U 2 (#caa171)\n\
///     R 2 (#7807d2)\n\
///     U 3 (#a77fa3)\n\
///     L 2 (#015232)\n\
///     U 2 (#7a21e3)";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "952408144115");
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let instructions = self.input.lines()
			.map(|line|
			{
				let (part1, part2) = line.rsplit_once(' ').ok_or_else(|| anyhow!("cannot split {:?} by space from the right", line))?;
				match self.part
				{
					Part::One =>
					{
						let (dir, num) = part1.split_once(' ').ok_or_else(|| anyhow!("cannot split {:?} by space", part1))?;
						anyhow::ensure!(dir.len() == 1, "direction needs to be 1 char, is {}", dir.len());
						let dir = match dir.chars().next().unwrap()
						{
							'U' => Direction::Up,
							'D' => Direction::Down,
							'L' => Direction::Left,
							'R' => Direction::Right,
							_ => Err(anyhow!("char {:?} does not correspond to a direction", dir))?,
						};
						let num = num.parse()?;
						Ok((dir, num))
					},
					Part::Two =>
					{
						let colour = part2.strip_prefix("(#").ok_or_else(|| anyhow!("cannot strip colour prefix from {:?}", part2))?.strip_suffix(')').ok_or_else(|| anyhow!("cannot strip colour suffix from {:?}", part2))?.to_string();
						let num = usize::from_str_radix(&colour, 16).context(anyhow!("cannot parse part2 colour: {:?}", colour))?;
						let dir = match num % 16
						{
							0 => Direction::Right,
							1 => Direction::Down,
							2 => Direction::Left,
							3 => Direction::Up,
							_ => bail!("part 2 colour ends with {}", num % 16),
						};
						Ok((dir, num / 16))
					},
				}
			})
			.inspect(|inst| trace!("parsed instruction: {:?}", inst))
			.collect::<Result<Vec<(Direction, usize)>>>()?;

		let offsets = instructions.iter()
			.fold(((0isize, 0isize), 0isize, 0isize, 0isize, 0isize), |(pos, min_y, min_x, max_y, max_x), &(dir, count)|
			{
				let (dy, dx) = dir.vector();
				let new_pos = (pos.0 + dy * count as isize, pos.1 + dx * count as isize);
				(new_pos, min_y.min(new_pos.0), min_x.min(new_pos.1), max_y.max(new_pos.0), max_x.max(new_pos.1))
			});

		let start = (-offsets.1 as usize, -offsets.2 as usize);
		let len = ((offsets.3 - offsets.1) as usize + 1, (offsets.4 - offsets.2) as usize + 1);

		debug!("field of size {}x{}", len.0, len.1);

		let mut field = std::collections::BTreeSet::<(usize, usize)>::new();

		field.insert(start);

		instructions.iter()
			.inspect(|inst| trace!("executing instruction: {:?}", inst))
			.fold(start, |pos, &(dir, count)|
			{
				let mut newpos = pos;
				for _ in 0..count
				{
					newpos = (newpos + dir).unwrap();
					field.insert(newpos);
				}
				newpos
			});

		if field.len() > 10000
		{
			debug!("field after digging: very big");
		}
		else
		{
			let field = &field;
			debug!("field after digging:\n{}",
				(0..len.0)
					.flat_map(|y|
					{
						(0..len.1)
							.map(move |x| field.contains(&(y, x)))
							.map(|b| if b { '#' } else { '.' })
							.chain(['\n'])
					})
					.collect::<String>()
			);
		}

		let mut rows = field.into_iter().fold(std::collections::BTreeMap::<usize, Vec<usize>>::new(), |mut rows, (y, x)|
		{
			rows.entry(y).and_modify(|v| v.push(x)).or_insert_with(|| vec![x]);
			rows
		});

		let mut removedrows = 0;

		for i in (0..(len.0 - 1)).rev()
		{
			if rows[&i] == rows[&(i+1)]
			{
				rows.remove(&(i+1));
				removedrows += 1;
			}
		}

		trace!("removed {} of {} rows", removedrows, len.1);

		let len = (len.0 - removedrows, len.1);

		let field: std::collections::BTreeSet<(usize, usize)> = rows.values()
			.enumerate()
			.flat_map(|(y, xvec)| xvec.iter().map(move |&x| (y, x)))
			.collect();

		let mut cols = field.into_iter().fold(std::collections::BTreeMap::<usize, Vec<usize>>::new(), |mut cols, (y, x)|
		{
			cols.entry(x).and_modify(|v| v.push(y)).or_insert_with(|| vec![y]);
			cols
		});

		let mut removedcols = 0;

		for i in (0..(len.1 - 1)).rev()
		{
			if cols[&i] == cols[&(i+1)]
			{
				cols.remove(&(i+1));
				removedcols += 1;
			}
		}

		trace!("removed {} of {} cols", removedcols, len.1);

		let len = (len.0, len.1 - removedcols);

		let field: std::collections::BTreeSet<(usize, usize)> = cols.values()
			.enumerate()
			.flat_map(|(x, yvec)| yvec.iter().map(move |&y| (y, x)))
			.collect();
		let field = &field;


		if field.len() > 10000
		{
			debug!("field after minimizing: still very big");
		}
		else
		{
			let field = &field;
			debug!("field after minimizing:\n{}",
				(0..len.0)
					.flat_map(|y|
					{
						(0..len.1)
							.map(move |x| field.contains(&(y, x)))
							.map(|b| if b { '#' } else { '.' })
							.chain(['\n'])
					})
					.collect::<String>()
			);
		}

		let mut cutouts: std::collections::BTreeSet::<(usize, usize)> = Default::default();
		let mut next: std::collections::BTreeSet::<(usize, usize)> = std::iter::empty()
			.chain((0..len.0).map(|y| (y, 0)))
			.chain((0..len.0).map(|y| (y, len.1 - 1)))
			.chain((0..len.1).map(|x| (0, x)))
			.chain((0..len.1).map(|x| (len.0 - 1, x)))
			.filter(|pos| !field.contains(pos))
			.collect();

		trace!("starting with {} cutouts", cutouts.len());

		loop
		{
			let clen = cutouts.len();
			let tmp = next.difference(&cutouts)
				.copied()
				.flat_map(|pos| Direction::slice().iter().copied().map(move |dir| (pos, dir)))
				.flat_map(|(pos, dir)|
				{
					let num = match dir
					{
						Direction::Up | Direction::Down =>
						{
							if let Some(pos) = pos + dir
							{
								(pos.0 < len.0 && pos.1 < len.1 && !field.contains(&pos)).into()
							}
							else
							{
								0
							}
						},
						Direction::Left =>
						{
							let until_field = field.range((Bound::Included((pos.0, 0)), Bound::Excluded(pos))).next_back().map(|(_, x)| pos.1.saturating_sub(x + 1)).unwrap_or(pos.1);
							let until_next = next.range((Bound::Included((pos.0, 0)), Bound::Excluded(pos))).next_back().map(|(_, x)| pos.1.saturating_sub(x + 1)).unwrap_or(pos.1);
							let until_cutouts = cutouts.range((Bound::Included((pos.0, 0)), Bound::Excluded(pos))).next_back().map(|(_, x)| pos.1.saturating_sub(x + 1)).unwrap_or(pos.1);
							let base = pos.1;
							base
								.min(until_field)
								.min(until_next)
								.min(until_cutouts)
						},
						Direction::Right =>
						{
							let until_field = field.range((Bound::Excluded(pos), Bound::Excluded((pos.0, len.1)))).next().map(|(_, x)| x - pos.1).unwrap_or(len.1 - pos.1) - 1;
							let until_next = next.range((Bound::Excluded(pos), Bound::Excluded((pos.0, len.1)))).next().map(|(_, x)| x - pos.1).unwrap_or(len.1 - pos.1) - 1;
							let until_cutouts = cutouts.range((Bound::Excluded(pos), Bound::Excluded((pos.0, len.1)))).next().map(|(_, x)| x - pos.1).unwrap_or(len.1 - pos.1) - 1;
							let base = len.1 - pos.1 - 1;
							base
								.min(until_field)
								.min(until_next)
								.min(until_cutouts)
						},
					};
					std::iter::successors(Some(pos), move |&pos| pos + dir)
						.skip(1)
						.take(num)
				})
				.collect::<std::collections::BTreeSet<_>>();
			cutouts.extend(next);
			next = tmp;

			trace!("new cutouts extended: +{} ({})", cutouts.len() - clen, cutouts.len());

			if cutouts.len() - clen == 0
			{
				break;
			}
		}

		let rowkeys = rows.keys().copied().collect::<Vec<_>>();
		let colkeys = cols.keys().copied().collect::<Vec<_>>();
		let sum: usize = rowkeys.windows(2)
			.enumerate()
			.map(|(idx, s)|
			{
				let multiplier = s[1] - s[0];
				let count: usize = cutouts.range((Bound::Included((idx, 0)), Bound::Included((idx, len.1))))
					.map(|&(_, x)|
					{
						if x == colkeys.len() - 1 { return 1; }
						let lower = colkeys[x];
						let higher = colkeys[x + 1];
						higher - lower
					})
					.sum();
				trace!("row {} (real: {}) has {} cutouts, next row is {}, multiplier: {}", idx, s[0], count, s[1], multiplier);
				count * multiplier
			})
			.chain(std::iter::once_with(||
			{
				let count: usize = cutouts.range((Bound::Included((len.0 - 1, 0)), Bound::Included((len.0 - 1, len.1))))
					.map(|&(_, x)|
					{
						if x == colkeys.len() - 1 { return 1; }
						let lower = colkeys[x];
						let higher = colkeys[x + 1];
						higher - lower
					})
					.sum();
				trace!("row {} (real: {}) has {} cutouts", len.0 - 1, len.0 - 1 + removedrows, count);
				count
			}))
			.sum();

		let result: usize = (len.0 + removedrows) * (len.1 + removedcols) - sum;

		Ok(format!("{}", result))
	}
}

