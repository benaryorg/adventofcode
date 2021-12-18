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
/// # use adventofcode::solution::{ y2021::d17pt2::Solution, Solution as S };
/// # env_logger::init();
/// assert_eq!(Solution::new("target area: x=20..30, y=-10..-5".to_string()).solve().unwrap(), "112");
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

		// this has a good chance of working, don't rely on it tho

		let (_, ((x_low, x_high), (y_low, y_high))) = terminated(input, eof)(self.input.trim())
			.map_err(|err| anyhow!("{}", err))
			.context("input not parsable")?;

		let y_far = if y_low.abs() >= y_high.abs() { y_low } else { y_high };
		let y_diff = y_far.abs() + y_far.is_negative() as isize;

		let count = (1..=x_high)
			.flat_map(|x| (-y_diff..y_diff).map(move |y| (x, y)))
			.filter(|&(x_vel, y_vel)|
			{
				(0..)
					.scan((0, 0), |(x, y), step|
					{
						*x += (x_vel - step).max(0);
						*y += y_vel - step;
						Some((*x, *y))
					})
					.take_while(|&(x, y)| y >= y_low && x <= x_high)
					.any(|(x, y)| x >= x_low && x <= x_high && y >= y_low && y <= y_high)
			})
			.count();

		Ok(format!("{}", count))
	}
}

