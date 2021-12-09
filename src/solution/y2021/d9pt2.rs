use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d9pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "2199943210\n\
///     3987894921\n\
///     9856789892\n\
///     8767896789\n\
///     9899965678";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "1134");
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut map = self.input.lines()
			.enumerate()
			.flat_map(|(y, line)|
			{
				line.chars()
					.enumerate()
					.map(move |(x, ch)|
					{
						let num = ch.to_digit(10)
							.ok_or(Error::AocParsing)
							.context(anyhow!("character is not a digit: '{}'", ch))?;
						Ok(((x as isize, y as isize), num as usize))
					})
			})
			.collect::<Result<std::collections::BTreeMap<(isize, isize), usize>>>()?;

		map.retain(|_, &mut height| height < 9);

		let surround = vec!
		[
			(-1, 0), // left
			(0, 1), // top
			(1, 0), // right
			(0, -1), // bottom
		];

		let mut sizes = Vec::new();

		while !map.is_empty()
		{
			let key = map.iter().max_by_key(|(_key, value)| *value).unwrap().0;
			let mut basin = std::collections::BTreeSet::new();
			basin.insert(*key);

			loop
			{
				let neighbours = basin.iter()
					.flat_map(|&(x, y)|
					{
						surround.iter()
							.map(|(dx, dy)| (dx + x, dy + y))
							.filter(|pos| map.contains_key(pos))
							.filter(|pos| !basin.contains(pos))
							.inspect(|pos| trace!("adding {:?} for {:?}", pos, (x, y)))
							.collect::<Vec<_>>()
					})
					.collect::<std::collections::BTreeSet<(isize, isize)>>();

				debug!("adding neighbours to basin: {:?}", neighbours);

				if neighbours.is_empty()
				{
					break;
				}
				basin.extend(neighbours);
			}

			info!("removing {} elements from map", basin.len());
			map.retain(|pos, _| !basin.contains(pos));
			sizes.push(basin.len());
		}

		sizes.sort();

		Ok(format!("{}", sizes.iter().rev().take(3).product::<usize>()))
	}
}

