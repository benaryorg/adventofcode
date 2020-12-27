use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D1Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "1721\n\
///     979\n\
///     366\n\
///     299\n\
///     675\n\
///     1456";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "514579");
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

		let numbers = self.input.lines().map(|s| Ok(s.parse::<usize>()?)).collect::<Result<Vec<_>>>()?;
		let target = 2020;
		let (low,high): (std::collections::BTreeSet<_>,std::collections::BTreeSet<_>) = numbers.into_iter().partition(|&i| i < target/2);
		for &low in std::iter::once(&0).chain(low.iter())
		{
			let result = high.iter()
				.take_while(|&high| high + low <= target)
				.find(|&high| high + low == target);

			if let Some(result) = result
			{
				return Ok(format!("{}", low.max(1) * result));
			}
		}

		bail!(ErrorKind::NoSolution);
	}
}

