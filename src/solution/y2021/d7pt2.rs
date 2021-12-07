use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d7pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "16,1,2,0,4,2,7,1,2,14";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "168");
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

		let positions = self.input
			.trim()
			.split(',')
			.map(|s| Ok(s.parse()?))
			.collect::<Result<Vec<usize>>>()?;

		let min = *positions.iter().min().ok_or(Error::AocNoSolution).context("no minimum")?;
		let max = *positions.iter().max().ok_or(Error::AocNoSolution).context("no maximum")?;

		let mut vec = vec![0;max - min + 1];

		for source in positions
		{
			for target in min..=max
			{
				let distance = source.max(target) - source.min(target);
				let fuel = (distance * (1 + distance)) / 2;
				vec[target - min] += fuel;
			}
		}

		let min = vec.into_iter().min().ok_or(Error::AocNoSolution).context("no maximum")?;

		Ok(format!("{}", min))
	}
}

