use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d1pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "142");
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

		let result = self.input.lines()
			.enumerate()
			.map(|(idx, line)|
			{
				let first = line.chars().find_map(|ch| ch.to_digit(10)).ok_or(Error::AocParsing)?;
				let last = line.chars().rev().find_map(|ch| ch.to_digit(10)).ok_or(Error::AocParsing)?;
				debug!("line {} ({:?}): first {}, last {}", idx, line, first, last);
				Ok(first * 10 + last)
			})
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.sum::<u32>();

		Ok(format!("{}", result))
	}
}

