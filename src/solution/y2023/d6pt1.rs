use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d6pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     Time:      7  15   30\n\
///     Distance:  9  40  200";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "288");
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

		let (time, distance) = self.input.split_once('\n').ok_or(Error::AocParsing)?;
		let time = time.strip_prefix("Time:").ok_or(Error::AocParsing)?;
		let distance = distance.strip_prefix("Distance:").ok_or(Error::AocParsing)?;

		let races = time.split_whitespace()
			.map(|s| s.parse())
			.zip(distance.split_whitespace().map(|s| s.parse()))
			.map(|(time, distance)| Ok((time?, distance?)))
			.collect::<Result<Vec<(usize, usize)>>>()
			.context(Error::AocParsing)?;

		let result = races.into_iter()
			.enumerate()
			.map(|(id, (time, dist))|
			{
				trace!("race {}: time={}, distance={}", id, time, dist);

				(1..time)
					.map(|held| (time - held) * held)
					.filter(|&time| time > dist)
					.count()
			})
			.product::<usize>();

		Ok(format!("{}", result))
	}
}

