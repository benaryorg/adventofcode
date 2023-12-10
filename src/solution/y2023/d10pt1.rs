use crate::error::*;

use std::convert::TryFrom;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d10pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     -L|F7\n\
///     7S-7|\n\
///     L|7||\n\
///     -L-J|\n\
///     L|-JF";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "4");
/// let input = "\
///     7-F7-\n\
///     .FJ|7\n\
///     SJLL7\n\
///     |F--J\n\
///     LJ.LJ";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "8");
/// ```
pub struct Solution
{
	input: String,
}

impl Solution
{
	pub fn new(input: String) -> Self
	{
		Self { input, }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile
{
	Start,
	Nothing,
	UpRight,
	RightDown,
	DownLeft,
	LeftUp,
	LeftRight,
	UpDown,
}

impl Tile
{
	fn path(&self) -> Option<&[(isize, isize)]>
	{
		match *self
		{
			Tile::Start => Some(&[(-1, 0), (0, 1), (0, -1), (1, 0)]),
			Tile::Nothing => None,
			Tile::UpRight => Some(&[(0, -1), (1, 0)]),
			Tile::RightDown => Some(&[(1, 0), (0, 1)]),
			Tile::DownLeft => Some(&[(0, 1), (-1, 0)]),
			Tile::LeftUp => Some(&[(-1, 0), (0, -1)]),
			Tile::LeftRight => Some(&[(-1, 0), (1, 0)]),
			Tile::UpDown => Some(&[(0, -1), (0, 1)]),
		}
	}
}

impl TryFrom<char> for Tile
{
	type Error = Error;
	fn try_from(ch: char) -> std::result::Result<Self, Error>
	{
		match ch
		{
			'S' => Ok(Tile::Start),
			'.' => Ok(Tile::Nothing),
			'L' => Ok(Tile::UpRight),
			'F' => Ok(Tile::RightDown),
			'7' => Ok(Tile::DownLeft),
			'J' => Ok(Tile::LeftUp),
			'-' => Ok(Tile::LeftRight),
			'|' => Ok(Tile::UpDown),
			_ => Err(Error::AocParsing),
		}
	}
}

fn pos_plus_dir(pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)>
{
	(pos.0).checked_add_signed(dir.0)
		.and_then(|x| pos.1.checked_add_signed(dir.1).map(|y| (x, y)))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let map = self.input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch| Ok(Tile::try_from(ch)?))
					.collect::<Result<Vec<Tile>>>()
			})
			.collect::<Result<Vec<Vec<Tile>>>>()?;

		let tiles = map.iter()
			.enumerate()
			.flat_map(|(y, row)|
			{
				row.iter()
					.enumerate()
					.map(move |(x, tile)| ((x, y), *tile))
			})
			.collect::<Vec<((usize, usize), Tile)>>();

		let start = tiles.iter().find(|(_, tile)| Tile::Start.eq(tile)).unwrap().0;

		let directions: &[(isize, isize)] = &[(-1, 0), (0, 1), (0, -1), (1, 0)];

		let path = directions.iter()
			.copied()
			.filter_map(|dir|
			{
				pos_plus_dir(start, dir)
			})
			.filter(|pos|
			{
				map.get(pos.1)
					.and_then(|row| row.get(pos.0))
					.and_then(|tile| tile.path())
					.map(|path|
					{
						path.iter()
							.any(|dir| pos_plus_dir(*pos, *dir) == Some(start))
					})
					.unwrap_or(false)
			})
			.filter_map(|entry|
			{
				debug!("trying start: {:?}", entry);
				let path = std::iter::successors(Some((entry, start)), |(pos@(x, y), last)|
					{
						let current = map[*y][*x];
						trace!("walk path: {:?} ({:?})", pos, current);
						if current == Tile::Start
						{
							trace!("is start, stopping");
							None
						}
						else
						{
							current.path()
								.unwrap()
								.iter()
								.filter_map(|dir| pos_plus_dir(*pos, *dir))
								.filter(|next| next != last)
								.inspect(|next| trace!("evaluating next: {:?}", next))
								.find(|next|
								{
									map.get(next.1)
										.and_then(|row| row.get(next.0))
										.and_then(|tile| tile.path())
										.map(|path|
										{
											path.iter()
												.any(|dir| pos_plus_dir(*next, *dir) == Some(*pos))
										})
										.unwrap_or(false)
								})
								.map(|next| (next, *pos))
						}
					})
					.map(|((x, y), _)| ((x, y), map[y][x]))
					.collect::<Vec<_>>();
				path.last()
					.copied()
					.and_then(|last|
					{
						(last.1 == Tile::Start).then_some(path)
					})
			})
			.next().unwrap();

		debug!("path found:\n{:#?}", path);

		let result: usize = (path.len() + 1) / 2;

		Ok(format!("{}", result))
	}
}

