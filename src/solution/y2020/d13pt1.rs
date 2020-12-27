use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D13Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "939\n7,13,x,x,59,x,31,19";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "295");
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
		let mut lines = self.input.lines();
		let start_time = lines.next().ok_or(ErrorKind::ParseError)?.parse::<usize>()?;
		let busses = lines.next()
			.ok_or(ErrorKind::ParseError)?
			.split(",")
			.filter(|&id| id != "x")
			.map(|id| Ok(id.parse::<usize>()?))
			.collect::<Result<Vec<_>>>()?;

		let bus = busses.into_iter().min_by_key(|id| id - start_time % id).ok_or(ErrorKind::NoSolution)?;

		Ok(format!("{}", (bus - start_time % bus) * bus))
	}
}

