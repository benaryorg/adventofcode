use crate::error::*;

use std::convert::TryFrom;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d10pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     ...........\n\
///     .S-------7.\n\
///     .|F-----7|.\n\
///     .||.....||.\n\
///     .||.....||.\n\
///     .|L-7.F-J|.\n\
///     .|..|.|..|.\n\
///     .L--J.L--J.\n\
///     ...........";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "4");
/// let input = "\
///     .F----7F7F7F7F-7....\n\
///     .|F--7||||||||FJ....\n\
///     .||.FJ||||||||L7....\n\
///     FJL7L7LJLJ||LJ.L-7..\n\
///     L--J.L7...LJS7F-7L7.\n\
///     ....F-J..F7FJ|L7L7L7\n\
///     ....L7.F7||L7|.L7L7|\n\
///     .....|FJLJ|FJ|F7|.LJ\n\
///     ....FJL-7.||.||||...\n\
///     ....L---J.LJ.LJLJ...";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "8");
/// let input = "\
///     FF7FSF7F7F7F7F7F---7\n\
///     L|LJ||||||||||||F--J\n\
///     FL-7LJLJ||||||LJL-77\n\
///     F--JF--7||LJLJ7F7FJ-\n\
///     L---JF-JLJ.||-FJLJJ7\n\
///     |F|F-JF---7F7-L7L|7|\n\
///     |FFJF7L7F-JF7|JL---7\n\
///     7-L-JL7||F7|L7F-7F7|\n\
///     L.L7LFJ|||||FJL7||LJ\n\
///     L7JLJL-JLJLJL--JLJ.L";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "10");
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

fn pos_minus_pos(dest: (usize, usize), source: (usize, usize)) -> (isize, isize)
{
	let x = dest.0.checked_sub(source.0).map(|i| i as isize).unwrap_or(-1);
	let y = dest.1.checked_sub(source.1).map(|i| i as isize).unwrap_or(-1);
	(x, y)
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

		trace!("path found:\n{:#?}", path);

		let pos_path = path.iter().map(|(pos, _)| pos).copied().collect::<Vec<_>>();

		let start = pos_path.last().copied().unwrap();
		let first = pos_path.first().copied().unwrap();
		let dir = pos_minus_pos(first, start);

		let dir = pos_path.iter()
			.skip(1)
			.copied()
			.fold((dir, first, 0isize), |(dir, last, count), next|
			{
				let new_dir = pos_minus_pos(next, last);
				// positive is left, negative is right
				let lr = match (new_dir, dir)
				{
					((1, 0), (0, 1)) => 1,
					((1, 0), (0, -1)) => -1,
					((0, 1), (1, 0)) => -1,
					((0, 1), (-1, 0)) => 1,
					((-1, 0), (0, 1)) => -1,
					((-1, 0), (0, -1)) => 1,
					((0, -1), (1, 0)) => 1,
					((0, -1), (-1, 0)) => -1,
					_ => 0,
				};
				if lr != 0
				{
					trace!("path is going {}", if lr > 0 { "left" } else { "right" });
				}
				(new_dir, next, count + lr)
			}).2.signum();

		debug!("path is {} bound", if dir > 0 { "left" } else { "right" });

		let enclosed = pos_path.iter()
			.skip(1)
			.chain(std::iter::once(&first))
			.copied()
			.fold((first, Vec::new()), |(last, mut tiles), next|
			{
				let new_dir = pos_minus_pos(next, last);
				// positive is left, negative is right
				trace!("evaluating new_dir={:?}, dir={}", new_dir, dir);
				let offset = match (new_dir, dir)
				{
					((1, 0), 1) => (0, -1),
					((-1, 0), 1) => (0, 1),
					((0, 1), 1) => (1, 0),
					((0, -1), 1) => (-1, 0),
					((1, 0), -1) => (0, 1),
					((-1, 0), -1) => (0, -1),
					((0, 1), -1) => (-1, 0),
					((0, -1), -1) => (1, 0),
					_ => unreachable!(),
				};
				if let Some(pos) = pos_plus_dir(last, offset)
				{
					trace!("adding tile {:?}", pos);
					tiles.push(pos);
				}
				if let Some(pos) = pos_plus_dir(next, offset)
				{
					trace!("adding tile {:?}", pos);
					tiles.push(pos);
				}
				(next, tiles)

			}).1;

		let mut enclosed = enclosed.into_iter()
			.filter(|pos| !pos_path.contains(pos))
			.collect::<std::collections::BTreeSet<_>>();

		debug!("found {} adjacent enclosed tiles", enclosed.len());

		loop
		{
			let len = enclosed.len();
			let new = enclosed.iter()
				.flat_map(|pos|
					directions.iter().filter_map(move |dir| pos_plus_dir(*pos, *dir))
				)
				.filter(|pos| !pos_path.contains(pos))
				.collect::<Vec<_>>();
			enclosed.extend(new);
			if len == enclosed.len()
			{
				break;
			}
		}

		trace!("enclosed tiles:\n{:#?}", enclosed);
		debug!("found {} total enclosed tiles", enclosed.len());

		let result: usize = enclosed.len();

		Ok(format!("{}", result))
	}
}

