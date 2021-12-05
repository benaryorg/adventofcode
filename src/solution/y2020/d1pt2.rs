use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D1Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "1721\n\
///     979\n\
///     366\n\
///     299\n\
///     675\n\
///     1456";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "241861950");
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

		let numbers = self.input.lines().map(|s| Ok(s.parse::<usize>()?)).collect::<Result<std::collections::BTreeSet<_>>>()?;
		let target = 2020;
		for &base in numbers.iter()
		{
			let target = target - base;
			for &low in numbers.range(..(target/2))
			{
				let target = target - low;
				if let Some(&high) = numbers.get(&target)
				{
					return Ok(format!("{}", base * low * high));
				}
			}
		}

		bail!(Error::AocNoSolution);
	}
}

