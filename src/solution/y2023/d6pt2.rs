use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d6pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     Time:      7  15   30\n\
///     Distance:  9  40  200";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "71503");
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

		let (time, dist) = self.input.split_once('\n').ok_or(Error::AocParsing)?;
		let time = time.strip_prefix("Time:").ok_or(Error::AocParsing)?.split_whitespace().flat_map(str::chars).collect::<String>().parse::<i64>().context(Error::AocParsing)?;
		let dist = dist.strip_prefix("Distance:").ok_or(Error::AocParsing)?.split_whitespace().flat_map(str::chars).collect::<String>().parse::<i64>().context(Error::AocParsing)?;

		debug!("race: time={}, dist={}", time, dist);

		/*
		 * math:
		 *  ((time - held) * held) = dist
		 *  time * held - held * held = dist
		 *  -held^2 + time * held = dist
		 *  -held^2 + time * held - dist = 0
		 * quadratic formula (https://en.wikipedia.org/wiki/Quadratic_formula):
		 *  ax^2 + bx + c
		 *  a = -1, b = time, c = -dist
		 */
		let a = (-1) as f64;
		let b = (time) as f64;
		let c = (-dist) as f64;

		// actual quadratic formula
		// oh also isqrt is nightly so this has to take a float detour
		//  let x1 = (-b (+-) (b.pow(2) - (4 * a * c)).isqrt()) / (2 * a);
		//  if future Katze ever looks at this and isqrt is stabilized, go ahead and change this
		let x1 = (-b + (b.powi(2) - (4f64 * a * c)).sqrt()) / (2f64 * a);
		let x2 = (-b - (b.powi(2) - (4f64 * a * c)).sqrt()) / (2f64 * a);

		// get them ordered
		let first = x1.min(x2).ceil() as isize;
		let last = x1.max(x2).trunc() as isize;

		debug!("first={} last={}", first, last);

		let result = (first..=last).count();

		Ok(format!("{}", result))
	}
}

