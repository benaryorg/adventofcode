use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d6pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "3,4,3,1,2";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "5934");
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

struct Fish
{
	timer: usize,
}

impl Fish
{
	fn new(timer: usize) -> Self
	{
		Self { timer, }
	}

	fn tick(&mut self) -> Option<Self>
	{
		if self.timer == 0
		{
			self.timer = 6;
			return Some(Self::new(8));
		}
		self.timer -= 1;
		None
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut fish = self.input.trim().split(',')
			.map(|num| Ok(Fish::new(num.parse()?)))
			.collect::<Result<Vec<_>>>()
			.context(Error::AocParsing).context("parsing of timers failed")?;

		for _ in 0..80
		{
			let new_fish = fish.iter_mut().flat_map(|fish| fish.tick()).collect::<Vec<_>>();
			fish.extend(new_fish);
		}

		Ok(format!("{}", fish.len()))
	}
}

