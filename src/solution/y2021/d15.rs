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
/// # use adventofcode::solution::{ y2021::d15::Solution, Solution as S };
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
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "40");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "315");
/// ```
pub struct Solution
{
	input: String,
	moar_input: bool,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			moar_input: false,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			moar_input: true,
		}
	}
}

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
		let (_, mut vec) = full_input(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		if self.moar_input
		{
			for inner in vec.iter_mut()
			{
				let clone = inner.clone();
				inner.extend(clone.iter().copied().map(|x| (x+1-1)%9+1));
				inner.extend(clone.iter().copied().map(|x| (x+2-1)%9+1));
				inner.extend(clone.iter().copied().map(|x| (x+3-1)%9+1));
				inner.extend(clone.iter().copied().map(|x| (x+4-1)%9+1));
			}
			let clone = vec.clone();
			vec.extend(clone.iter().cloned().map(|inner| inner.iter().map(|x| (x+1-1)%9+1).collect::<Vec<_>>()));
			vec.extend(clone.iter().cloned().map(|inner| inner.iter().map(|x| (x+2-1)%9+1).collect::<Vec<_>>()));
			vec.extend(clone.iter().cloned().map(|inner| inner.iter().map(|x| (x+3-1)%9+1).collect::<Vec<_>>()));
			vec.extend(clone.iter().cloned().map(|inner| inner.iter().map(|x| (x+4-1)%9+1).collect::<Vec<_>>()));
		}

		let (max_x, max_y) = (vec.len() as isize, vec.len() as isize);

		debug!("{}", vec.iter().map(|vec| vec.iter().copied().map(|i| char::from_digit(i as u32, 10).unwrap_or('?')).chain(std::iter::once('\n')).collect::<String>()).collect::<String>());

		let risks = vec.into_iter()
			.flat_map(|vec| vec.into_iter())
			.collect::<Vec<isize>>();

		let connections =
		[
			(-1, 0), // left
			(0, 1), // top
			(1, 0), // right
			(0, -1), // bottom
		];

		debug!("max: ({}, {})", max_x, max_y);
		let num_elems = (max_x * max_y) as usize;

		let mut distance: Vec<Option<(bool, isize)>> = vec![None; num_elems];
		let mut current = (0, 0);

		distance[0] = Some((false, 0));

		loop
		{
			let (x, y) = current;
			let current_distance = distance.get((x+y*max_x) as usize)
				.ok_or(Error::AocNoSolution).with_context(|| anyhow!("cannot find current ({}, {}) in distance map", x, y))?
				.ok_or(Error::AocNoSolution).with_context(|| anyhow!("current ({}, {}) is None in distance map", x, y))?
				.1;

			debug!("checking neighbours for ({}, {}) with distance {}", x, y, current_distance);

			for (nx, ny) in connections.iter()
				.copied()
				.map(|(dx, dy)| (x + dx, y + dy))
				.filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < max_x && ny < max_y)
			{
				trace!("checking neighbour ({}, {})", nx, ny);
				let risk = risks.get((nx+ny*max_x) as usize)
					.ok_or(Error::AocNoSolution)
					.with_context(|| anyhow!("cannot find neighbour ({}, {}) in risk map", x, y))?;
				trace!("neighbour has risk {}", risk);
				let entry = distance.get_mut((nx+ny*max_x) as usize)
					.ok_or(Error::AocNoSolution)
					.with_context(|| anyhow!("cannot find neighbour ({}, {}) in distance map", x, y))?;
				if let Some((visited, value)) = entry
				{
					if *visited
					{
						continue;
					}
					*value = (current_distance + risk).min(*value);
				}
				else
				{
					*entry = Some((entry.map(|(visited, _)| visited).unwrap_or(false), current_distance + risk));
				}
				trace!("neighbour has distance entry {:?}", entry);
			}

			if current == (max_x - 1, max_y - 1)
			{
				break;
			}
			distance.get_mut((x+y*max_x) as usize).unwrap().as_mut().unwrap().0 = true;
			current = distance.iter()
				.enumerate()
				.filter_map(|(pos, opt)|
				{
					if let Some((visited, distance)) = opt
					{
						if *visited
						{
							None
						}
						else
						{
							Some(((pos as isize % max_x, pos as isize / max_x), distance))
						}
					}
					else
					{
						None
					}
				})
				.inspect(|(pos, risk)| trace!("pos {:?} has risk {:?}", pos, risk))
				.min_by_key(|(_, risk)| *risk)
				.map(|(pos, _)| pos)
				.ok_or(Error::AocNoSolution)
				.context("cannot find next unvisited")?;
		}

		Ok(format!("{}", distance.last().ok_or(Error::AocNoSolution).context("last element does not have distance")?.unwrap().1))
	}
}

