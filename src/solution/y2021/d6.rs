use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d6::Solution, Solution as S };
/// # env_logger::init();
/// let input = "3,4,3,1,2";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "5934");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "26984457539");
/// ```
pub struct Solution
{
	input: String,
	days: usize,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			days: 80,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			days: 256,
		}
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let fish = self.input.trim().split(',')
			.map(|num| Ok(num.parse()?))
			.collect::<Result<Vec<_>>>()
			.context(Error::AocParsing).context("parsing of timers failed")?;

		let mut tank = std::collections::VecDeque::from(vec![0;9]);

		for fish in fish
		{
			tank[fish] += 1;
		}

		for _ in 0..self.days
		{
			let new_fish = tank.pop_front().unwrap_or(0);
			tank.push_back(new_fish);
			tank[6] += new_fish;

			debug!("{:?}", tank);
		}

		Ok(format!("{}", tank.into_iter().sum::<usize>()))
	}
}

