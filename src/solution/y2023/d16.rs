use crate::error::*;

use std::convert::TryFrom;

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d16::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     .|...\\....\n\
///     |.-.\\.....\n\
///     .....|-...\n\
///     ........|.\n\
///     ..........\n\
///     .........\\\n\
///     ..../.\\\\..\n\
///     .-.-/..|..\n\
///     .|....-|.\\\n\
///     ..//.|....";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "46");
/// ```
///
/// Part 2:
///
/// ```
/// # use adventofcode::solution::{ y2023::d16::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     .|...\\....\n\
///     |.-.\\.....\n\
///     .....|-...\n\
///     ........|.\n\
///     ..........\n\
///     .........\\\n\
///     ..../.\\\\..\n\
///     .-.-/..|..\n\
///     .|....-|.\\\n\
///     ..//.|....";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "51");
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Tile
{
	MirrorSlash,
	MirrorBackslash,
	SplitterDash,
	SplitterPipe,
	Empty,
}

impl Tile
{
	fn interact(&self, dir: Direction) -> &[Direction]
	{
		match (&self, dir)
		{
			(Tile::MirrorSlash, Direction::Right) => &[Direction::Up],
			(Tile::MirrorSlash, Direction::Left) => &[Direction::Down],
			(Tile::MirrorSlash, Direction::Down) => &[Direction::Left],
			(Tile::MirrorSlash, Direction::Up) => &[Direction::Right],
			(Tile::MirrorBackslash, Direction::Right) => &[Direction::Down],
			(Tile::MirrorBackslash, Direction::Left) => &[Direction::Up],
			(Tile::MirrorBackslash, Direction::Down) => &[Direction::Right],
			(Tile::MirrorBackslash, Direction::Up) => &[Direction::Left],
			(Tile::SplitterDash, Direction::Left) => &[Direction::Left],
			(Tile::SplitterDash, Direction::Right) => &[Direction::Right],
			(Tile::SplitterDash, Direction::Up | Direction::Down) => &[Direction::Left, Direction::Right],
			(Tile::SplitterPipe, Direction::Up) => &[Direction::Up],
			(Tile::SplitterPipe, Direction::Down) => &[Direction::Down],
			(Tile::SplitterPipe, Direction::Left | Direction::Right) => &[Direction::Up, Direction::Down],
			(Tile::Empty, Direction::Left) => &[Direction::Left],
			(Tile::Empty, Direction::Right) => &[Direction::Right],
			(Tile::Empty, Direction::Up) => &[Direction::Up],
			(Tile::Empty, Direction::Down) => &[Direction::Down],
		}
	}
}

impl TryFrom<char> for Tile
{
	type Error = Error;
	fn try_from(ch: char) -> std::result::Result<Self, Error>
	{
		Ok(match ch
		{
			'/' => Tile::MirrorSlash,
			'\\' => Tile::MirrorBackslash,
			'-' => Tile::SplitterDash,
			'|' => Tile::SplitterPipe,
			'.' => Tile::Empty,
			ch => Err(anyhow!("char {:?} cannot be converted to tile", ch))?,
		})
	}
}

fn calculate(field: &[&[Tile]], (pos, dir): ((usize, usize), Direction)) -> std::collections::BTreeSet<(Position, Direction)>
{
	let mut memo: std::collections::BTreeSet<(Position, Direction)> = Default::default();
	let mut positions = field.get(pos.1)
		.and_then(|row| row.get(pos.0))
		.unwrap()
		.interact(dir)
		.iter()
		.map(|&dir| (pos, dir))
		.collect::<Vec<_>>();

	while let Some((pos, dir)) = positions.pop()
	{
		if !memo.insert((pos, dir))
		{
			continue;
		}
		if let Some(npos) = pos + dir
		{
			trace!("going from {:?} {:?} to {:?}", pos, dir, npos);
			let pos = npos;
			if let Some(next) = field.get(pos.1)
				.and_then(|row| row.get(pos.0))
			{
				positions.extend(next.interact(dir).iter().map(|&dir| (pos, dir)))
			}
		}
	}

	memo
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let field = self.input.lines()
			.map(|line|
			{
				let res = line.chars().map(Tile::try_from).collect::<std::result::Result<Vec<Tile>, Error>>();
				Result::from(res).context(Error::AocParsing)
			})
			.collect::<Result<Vec<Vec<Tile>>>>()?;

		debug!("field:\n{:?}", field);

		let field: &[&[Tile]] = &field.iter()
			.map(|row| row.as_slice())
			.collect::<Vec<_>>();

		let result: usize = match self.part
		{
			Part::One => calculate(field, ((0, 0), Direction::Right)).into_iter()
				.map(|(pos, _)| pos)
				.collect::<std::collections::BTreeSet<_>>()
				.len(),
			Part::Two =>
			{
				std::iter::empty()
					.chain((0..field.len()).map(|y| ((0, y), Direction::Right)))
					.chain((0..field.len()).map(|y| ((field.len() - 1, y), Direction::Left)))
					.chain((0..field.len()).map(|x| ((x, 0), Direction::Down)))
					.chain((0..field.len()).map(|x| ((x, field.len() - 1), Direction::Up)))
					.map(|entry|
					{
						let len = calculate(field, entry).into_iter()
							.map(|(pos, _)| pos)
							.collect::<std::collections::BTreeSet<_>>()
							.len();
						debug!("walking from {:?} yields {}", entry, len);
						len
					})
					.max()
					.unwrap()
			},
		};
		
		Ok(format!("{}", result))
	}
}

