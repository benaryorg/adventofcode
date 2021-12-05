use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D10Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "28\n\
/// #   33\n\
/// #   18\n\
/// #   42\n\
/// #   31\n\
/// #   14\n\
/// #   46\n\
/// #   20\n\
/// #   48\n\
/// #   47\n\
/// #   24\n\
/// #   23\n\
/// #   49\n\
/// #   45\n\
/// #   19\n\
/// #   38\n\
/// #   39\n\
/// #   11\n\
/// #   1\n\
/// #   32\n\
/// #   25\n\
/// #   35\n\
/// #   8\n\
/// #   17\n\
/// #   7\n\
/// #   9\n\
/// #   4\n\
/// #   2\n\
/// #   34\n\
/// #   10\n\
///     3";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "220");
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
		let adapters = self.input.lines()
			.map(|line| Ok(line.parse()?))
			.collect::<Result<std::collections::BTreeSet<usize>>>()?;

		let device = adapters.iter().max().ok_or(Error::AocNoSolution)? + 3;

		let mut ones = 0;
		let mut threes = 0;
		for (before,after) in std::iter::once(&0)
			.chain(adapters.iter())
			.zip(adapters.iter().chain(std::iter::once(&device)))
		{
			match after-before
			{
				0 => {},
				1 => ones += 1,
				2 => {},
				3 => threes += 1,
				_ => bail!(Error::AocNoSolution),
			}
		}

		Ok(format!("{}", ones*threes))
	}
}

