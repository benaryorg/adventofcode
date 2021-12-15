use crate::error::*;

use nom::
{
	character::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d15pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "1163751742\n\
///     1381373672\n\
///     2136511328\n\
///     3694931569\n\
///     7463417111\n\
///     1319128137\n\
///     1359912421\n\
///     3125421639\n\
///     1293138521\n\
///     2311944581\n";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "40");
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

type Map<K, V> = std::collections::HashMap<K, V>;
type Set<V> = std::collections::HashSet<V>;

fn line(input: &str) -> IResult<&str, Vec<isize>>
{
	terminated(many1(map(one_of("0123456789"), |b| (b as isize - b'0' as isize))), newline)(input)
}

fn full_input(input: &str) -> IResult<&str, Vec<Vec<isize>>>
{
	terminated(many1(line), eof)(input)
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);
		let (_, vec) = full_input(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let risks = vec.into_iter()
			.enumerate()
			.flat_map(|(y, vec)| vec.into_iter().enumerate().map(move |(x, val)| ((x as isize, y as isize), val)))
			.collect::<Map<(isize, isize), isize>>();

		let connections =
		[
			(-1, 0), // left
			(0, 1), // top
			(1, 0), // right
			(0, -1), // bottom
		];

		let (max_x, max_y) = risks.keys().max().copied().ok_or(Error::AocNoSolution)?;

		let mut visited = Set::<(isize, isize)>::new();
		let mut distance = (0..=max_x).flat_map(|x| (0..=max_y).map(move |y| ((x, y), None))).collect::<Map<(isize, isize), Option<isize>>>();
		let mut current = (0, 0);

		distance.insert((0, 0), Some(0));

		while !visited.contains(&(max_x, max_y))
		{
			let (x, y) = current;
			let current_distance = distance.get(&(x, y)).ok_or(Error::AocNoSolution)?.unwrap();
			for (nx, ny) in connections.iter()
				.copied()
				.map(|(dx, dy)| (x + dx, y + dy))
				.filter(|ncoords| !visited.contains(ncoords))
				.filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx <= max_x && ny <= max_y)
			{
				let risk = risks.get(&(nx, ny)).ok_or(Error::AocNoSolution)?;
				let entry = distance.entry((nx, ny))
					.or_insert(None)
					.get_or_insert(current_distance + risk);
				if *entry > current_distance + risk
				{
					*entry = current_distance + risk;
				}
			}

			visited.insert(current);
			current = distance.iter()
				.filter(|(pos, _)| !visited.contains(pos))
				.filter(|(_, risk)| risk.is_some())
				.min_by_key(|(_, risk)| risk.unwrap())
				.map(|(pos, _)| pos)
				.copied()
				.ok_or(Error::AocNoSolution)?;
		}

		Ok(format!("{}", distance.get(&(max_x, max_y)).ok_or(Error::AocNoSolution)?.unwrap()))
	}
}

