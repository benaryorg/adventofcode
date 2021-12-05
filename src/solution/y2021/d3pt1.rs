use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d3pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "00100\n\
///     11110\n\
///     10110\n\
///     10111\n\
///     10101\n\
///     01111\n\
///     00111\n\
///     11100\n\
///     10000\n\
///     11001\n\
///     00010\n\
///     01010";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "198");
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

		let mut map = std::collections::BTreeMap::<usize, isize>::new();

		for line in self.input.lines()
		{
			for (pos, bit) in line.chars().rev().enumerate()
			{
				let value = match bit
				{
					'0' => false,
					'1' => true,
					_ => Err(Error::AocParseError).context("bit was neither 0 nor 1")?,
				};

				*map.entry(pos).or_default() += if value { 1 } else { -1 };
			}
		}

		let (gamma, epsilon) = map.iter().fold((0, 0), |(gamma, epsilon), (&pos, &value)|
		{
			let bit = value > 0;
			let gamma = gamma + ((bit as isize) << pos);
			let epsilon = epsilon + ((!bit as isize) << pos);
			(gamma, epsilon)
		});

		Ok(format!("{}", gamma*epsilon))
	}
}

