use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d1pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "281");
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

		const NUMBER_STRINGS: [&str;10] = [ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

		let result = self.input.lines()
			.enumerate()
			.map(|(idx, line)|
			{
				let first = (0..(line.len()))
					.map(|start| line.chars().skip(start).collect::<String>())
					.find_map(|substr|
					{
						let result = if substr.starts_with(|ch: char| ch.is_ascii_digit())
						{
							Some(substr.chars().next().unwrap().to_digit(10).unwrap() as usize)
						}
						else
						{
							NUMBER_STRINGS.iter().enumerate().find_map(|(num, num_str)| substr.starts_with(num_str).then_some(num))
						};
						trace!("first: inspecting substr {:?} => {:?}", substr, result);
						result
					})
					.ok_or(Error::AocParsing)?;

				let last = (1..=(line.len())).rev()
					.map(|end| line.chars().take(end).collect::<String>())
					.find_map(|substr|
					{
						let result = if substr.ends_with(|ch: char| ch.is_ascii_digit())
						{
							Some(substr.chars().last().unwrap().to_digit(10).unwrap() as usize)
						}
						else
						{
							NUMBER_STRINGS.iter().enumerate().find_map(|(num, num_str)| substr.ends_with(num_str).then_some(num))
						};
						trace!("last: inspecting substr {:?} => {:?}", substr, result);
						result
					})
					.ok_or(Error::AocParsing)?;

				debug!("line {} ({:?}): first {}, last {}", idx, line, first, last);
				Ok(first * 10 + last)
			})
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.sum::<usize>();

		Ok(format!("{}", result))
	}
}

