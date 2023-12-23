use crate::error::*;

/// This makes some assumptions:
/// 
/// - an intersection will never only have one destination (i.e. an intersection will not be a slope itself)
/// - the only passages that end nowhere are beginning and end
/// 
/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d23::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     ##.#####################\n\
///     ##.......#########...###\n\
///     ########.#########.#.###\n\
///     ####.....#.>.>.###.#.###\n\
///     ####v#####.#v#.###.#.###\n\
///     ####.>...#.#.#.....#...#\n\
///     ####v###.#.#.#########.#\n\
///     ####...#.#.#.......#...#\n\
///     ######.#.#.#######.#.###\n\
///     ##.....#.#.#.......#...#\n\
///     ##.#####.#.#.#########v#\n\
///     ##.#...#...#...###...>.#\n\
///     ##.#.#v#######v###.###v#\n\
///     ##...#.>.#...>.>.#.###.#\n\
///     ######v#.#.###v#.#.###.#\n\
///     ##.....#...#...#.#.#...#\n\
///     ##.#########.###.#.#.###\n\
///     ##...###...#...#...#.###\n\
///     ####.###.#.###v#####v###\n\
///     ##...#...#.#.>.>.#.>.###\n\
///     ##.###.###.#.###.#.#v###\n\
///     ##.....###...###...#...#\n\
///     ######################.#";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "94");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "154");
/// ```
pub struct Solution
{
	input: String,
	part: AocPart,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self { part: AocPart::One, input, }
	}

	pub fn part2(input: String) -> Self
	{
		Self { part: AocPart::Two, input, }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AocPart
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Tile
{
	Slope(Direction),
	Path,
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input:\n{}", self.input);

		let basic_grid: std::collections::BTreeMap<Position, Tile> = self.input.lines()
			.enumerate()
			.flat_map(|(y, line)|
			{
				line.chars()
					.enumerate()
					.filter(|&(_, ch)| ch != '#')
					.map(move |(x, ch)|
					{
						let tile = if ch == '.' || self.part == AocPart::Two
						{
							Tile::Path
						}
						else
						{
							Direction::slice().iter()
								.copied()
								.find(|dir| dir.symbol() == ch)
								.map(Tile::Slope)
								.ok_or_else(|| anyhow!("cannot find Tile for {:?}", ch))?
						};
						Ok(((x, y), tile))
					})
			})
			.collect::<Result<_>>()?;

		let entry = *basic_grid.first_key_value().unwrap().0;
		debug!("entry: {:?}", entry);
		let exit = *basic_grid.last_key_value().unwrap().0;
		debug!("exit: {:?}", exit);

		let grid: std::collections::BTreeMap<Position, Vec<(usize, Position)>> = basic_grid.iter()
			.filter_map(|(&pos, &tile)| (tile == Tile::Path).then_some(pos))
			// get intersections only
			.filter(|&pos|
			{
				Direction::slice().iter()
					.filter_map(|&dir| pos + dir)
					.filter(|npos| basic_grid.contains_key(npos))
					.count() > 2
			})
			.chain([entry])
			.map(|pos|
			{
				trace!("tracing from {:?}", pos);
				let v = Direction::slice().iter()
					.filter_map(|&dir| pos + dir)
					.filter(|&npos|
					{
						if let Some(tile) = basic_grid.get(&npos)
						{
							match tile
							{
								Tile::Slope(dir) => (npos + *dir) != Some(pos),
								Tile::Path => true,
							}
						}
						else
						{
							false
						}
					})
					.filter_map(|npos|
					{
						let mut path = std::collections::BTreeSet::<Position>::from([pos, npos]);
						std::iter::successors(Some(npos), |&pos|
							{
								path.insert(pos);
								match *basic_grid.get(&pos)?
								{
									Tile::Slope(dir) => pos + dir,
									Tile::Path =>
									{
										let mut next = Direction::slice().iter()
											.filter_map(|&dir| pos + dir)
											.filter(|npos| !path.contains(npos))
											.filter(|npos| basic_grid.contains_key(npos));
										let npos = next.next()?;
										next.next().is_none().then_some(npos)
									},
								}
							})
							.enumerate()
							.last()
							.filter(|&(_, pos)| pos != entry)
							.map(|(dist, pos)| (dist + 1, pos))
					})
					.collect();
				trace!("trace from {:?}: {:?}", pos, v);
				(pos, v)
			})
			.collect();

		trace!("grid: {:?}", grid);

		let mut finished = Vec::new();
		let mut paths = std::collections::VecDeque::from([(0, vec![entry])]);
		while let Some((dist, path)) = paths.pop_front()
		{
			let last = path.last().unwrap();
			if let Some(next) = grid.get(last)
			{
				for &(ndist, npos) in next
				{
					if !path.contains(&npos)
					{
						let npath = path.iter().copied().chain([npos]).collect();
						paths.push_back((dist + ndist, npath));
					}
				}
			}
			else
			{
				debug!("found path of length {} via {:?}", dist, path);
				finished.push((dist, path));
			}
		}
		let result: usize = finished.into_iter().map(|(distance, _)| distance).max().unwrap();
		
		Ok(format!("{}", result))
	}
}

