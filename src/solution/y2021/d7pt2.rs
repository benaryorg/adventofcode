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

		let fuel_usage = (0..(max - min + 1))
			.into_iter()
			.scan(0, |state, x|
			{
				*state += x;
				Some(*state)
			})
			.collect::<Vec<_>>();

		let vec = positions.into_iter().fold(vec![0;max - min + 1], |mut vec, position|
		{
			let preceeding = fuel_usage[..position].iter().rev();
			let succeeding = fuel_usage[1..].iter();

			for (sum, usage) in vec.iter_mut().zip(preceeding.chain(succeeding))
			{
				*sum += usage;
			}

			vec
		});

		let min = vec.into_iter().min().ok_or(Error::AocNoSolution).context("no maximum")?;

		Ok(format!("{}", min))
	}
}

