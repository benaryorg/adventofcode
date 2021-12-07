use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d7pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "16,1,2,0,4,2,7,1,2,14";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "37");
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

		let mut map = std::collections::BTreeMap::<usize, usize>::new();

		for source in positions
		{
			for target in min..=max
			{
				*map.entry(target).or_default() += source.max(target) - source.min(target);
			}
		}

		let min = map.iter().min_by_key(|(_key, value)| *value).ok_or(Error::AocNoSolution).context("no maximum")?;

		Ok(format!("{}", min.1))
	}
}

