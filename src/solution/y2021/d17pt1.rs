use crate::error::*;

use nom::
{
	character::complete::*,
	number::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d17pt1::Solution, Solution as S };
/// # env_logger::init();
/// assert_eq!(Solution::new("target area: x=20..30, y=-10..-5".to_string()).solve().unwrap(), "45");
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

fn input(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))>
{
	let (input, _) = tag("target area: x=")(input)?;
	let (input, x_low) = double(input)?;
	let (input, _) = many1(char('.'))(input)?;
	let (input, x_high) = double(input)?;
	let (input, _) = tag(", y=")(input)?;
	let (input, y_low) = double(input)?;
	let (input, _) = many1(char('.'))(input)?;
	let (input, y_high) = double(input)?;
	let (input, _) = opt(newline)(input)?;
	Ok((input, ((x_low as isize, x_high as isize), (y_low as isize, y_high as isize))))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		// NOTE: this has a good chance of working, don't rely on it tho

		let (_, ((x_low, x_high), (y_low, y_high))) = terminated(input, eof)(self.input.trim())
			.map_err(|err| anyhow!("{}", err))
			.context("input not parsable")?;

		let y_diff =
		{
			let (y_low, y_high) = (y_low.min(y_high), y_low.max(y_high));
			let y_far = if y_low.abs() >= y_high.abs() { y_low } else { y_high };
			y_far.abs() as usize + y_far.is_negative() as usize
		};

		let vel = (1..=x_high)
			.map(move |x_vel|
			{
				let max_steps = (1..=x_vel).rev()
					.scan(0, |a, b| { *a += b; Some(*a) })
					.enumerate()
					.filter(move |&(_, distance)| distance >= x_low && distance <= x_high)
					.map(|(steps, _)| ((steps + 1) as isize))
					.max()
					.unwrap_or(0);

				if x_vel == max_steps
				{
					y_diff
				}
				else
				{
					0
				}
			})
			.max()
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", (1..=(vel - 2)).sum::<usize>()))
	}
}

