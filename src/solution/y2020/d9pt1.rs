use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D9Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "35\n\
///     20\n\
///     15\n\
///     25\n\
///     47\n\
///     40\n\
///     62\n\
///     55\n\
///     65\n\
///     95\n\
///     102\n\
///     117\n\
///     150\n\
///     182\n\
///     127\n\
///     219\n\
///     299\n\
///     277\n\
///     309\n\
///     576";
/// assert_eq!(Solution::new(input.to_string(), 5).solve().expect("1"), "127");
/// ```
pub struct Solution
{
	input: String,
	preamble_length: usize,
}

impl Solution
{
	pub fn new(input: String, preamble_length: usize) -> Self
	{
		Self { input, preamble_length, }
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let numbers = self.input.lines()
			.map(|line| Ok(line.parse()?))
			.collect::<Result<Vec<isize>>>()?;

		let result = numbers.windows(self.preamble_length+1)
			.find(|input|
			{
				let (result,haystack) = input.split_last().unwrap();
				let haystack = haystack.iter().copied().collect::<std::collections::BTreeSet<_>>();
				!haystack.iter().any(|&needle| haystack.contains(&(result - needle)))
			})
			.ok_or(Error::AocNoSolution)?.last().unwrap();

		Ok(format!("{}", result))
	}
}

