use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d9pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     0 3 6 9 12 15\n\
///     1 3 6 10 15 21\n\
///     10 13 16 21 30 45";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "114");
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

		let inputs = self.input.lines()
			.map(|line|
			{
				trace!("parsing line {:?}", line);
				line.split_whitespace()
					.map(|s| Ok(s.parse::<isize>()?))
					.collect::<Result<Vec<isize>>>()
			})
			.collect::<Result<Vec<Vec<isize>>>>()?;

		let result: isize = inputs.into_iter()
			.map(|input|
			{
				std::iter::successors(Some(input), |input: &Vec<isize>|
					{
						Some(input.windows(2)
							.map(|s| s[1] - s[0])
							.collect::<Vec<_>>())
					})
					.inspect(|input| trace!("got: {:?}", input))
					.take_while(|v| v.iter().any(|&i| i != 0))
					.map(|v| v.last().copied().unwrap())
					.sum::<isize>()
			})
			.inspect(|i| debug!("got {}", i))
			.sum();

		Ok(format!("{}", result))
	}
}

