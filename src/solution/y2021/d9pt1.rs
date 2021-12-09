use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d9pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "2199943210\n\
///     3987894921\n\
///     9856789892\n\
///     8767896789\n\
///     9899965678";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "15");
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

		let map = self.input.lines()
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

		// code for diagonals:
		//let surround = (-1..1).map(|y| (-1..1).map(|x| (x, y))).collect::<Vec<_>>();
		let surround = vec!
		[
			(-1, 0), // left
			(0, 1), // top
			(1, 0), // right
			(0, -1), // bottom
		];

		let risk_sum = map.iter()
			.filter(|((x, y), height)|
			{
				surround.iter()
					.flat_map(|(dx, dy)|
					{
						map.get(&(dx + x, dy + y))
					})
					.all(|neighbour| neighbour > height)
			})
			.inspect(|(pos, height)| debug!("found low point at {:?} with height {}", pos, height))
			.map(|(_, height)| height + 1)
			.sum::<usize>();

		Ok(format!("{}", risk_sum))
	}
}

