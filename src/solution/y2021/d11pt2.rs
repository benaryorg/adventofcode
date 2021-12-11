use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d11pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "5483143223\n\
///     2745854711\n\
///     5264556173\n\
///     6141336146\n\
///     6357385478\n\
///     4167524645\n\
///     2176841721\n\
///     6882881134\n\
///     4846848554\n\
///     5283751526";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "195");
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

		let list = self.input.lines()
			.enumerate()
			.map(|(y, line)|
			{
				Ok(line.chars()
					.enumerate()
					.map(|(x, ch)|
					{
						let num = ch.to_digit(10).ok_or(Error::AocParsing)?;
						Ok(((x as isize, y as isize), (num as usize, false)))
					})
					.collect::<Result<Vec<_>>>()?)
			})
			.collect::<Result<Vec<_>>>()?;
		let mut map = list.into_iter().flatten().collect::<std::collections::BTreeMap<_, _>>();

		let surround =
		[
			(-1, -1),
			(-1, 0),
			(-1, 1),
			(0, -1),
			(0, 1),
			(1, -1),
			(1, 0),
			(1, 1),
		];

		for step in 1..
		{
			for (ref mut energy, _) in map.values_mut()
			{
				*energy += 1;
			}

			while map.values().any(|&(energy, done)| energy > 9 && !done)
			{
				let flashes = map.iter()
					.filter(|((_x, _y), (energy, done))| *energy > 9 && !done)
					.map(|(&(x, y), _)| (x, y))
					.collect::<Vec<_>>();

				for (x, y) in flashes.into_iter()
				{
					surround.iter()
						.for_each(|(dx, dy)|
						{
							if let Some((ref mut energy, _done)) = map.get_mut(&(dx + x, dy + y))
							{
								*energy += 1;
							}
						});

					map.get_mut(&(x, y)).unwrap().1 = true;
				}
			}

			if map.values().all(|(_, done)| *done)
			{
				return Ok(format!("{}", step));
			}

			for (ref mut energy, ref mut done) in map.values_mut()
			{
				if *done
				{
					*done = false;
					*energy = 0;
				}
			}
		}

		bail!(Error::AocNoSolution);
	}
}

