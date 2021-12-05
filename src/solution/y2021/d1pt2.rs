use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2021::D1Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "199\n\
///     200\n\
///     208\n\
///     210\n\
///     200\n\
///     207\n\
///     240\n\
///     269\n\
///     260\n\
///     263";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "5");
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

		let numbers = self.input
			.lines()
			.map(|s| s.parse::<usize>().context("parsing line to number"))
			.collect::<Result<Vec<_>>>()?;

		let window_sums = numbers.windows(3).map(|win| win.iter().sum()).collect::<Vec<usize>>();

		let bump_up = window_sums.windows(2)
			// get all increasing windows
			.filter(|slice| slice[1] > slice[0])
			.count();

		Ok(format!("{}", bump_up))
	}
}

