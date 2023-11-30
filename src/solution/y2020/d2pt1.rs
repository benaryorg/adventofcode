use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D2Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "1-3 a: abcde\n\
///     1-3 b: cdefg\n\
///     2-9 c: ccccccccc";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "2");
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

struct PasswordPolicy<R>
{
	range: R,
	character: char,
}

impl<R> From<(R, char)> for PasswordPolicy<R>
{
	fn from(other: (R, char)) -> Self
	{
		PasswordPolicy
		{
			range: other.0,
			character: other.1,
		}
	}
}

impl<R> PasswordPolicy<R>
	where
		R: std::ops::RangeBounds<usize>
{
	fn check<S: AsRef<str>>(&self, s: S) -> bool
	{
		let max_cap: usize = match self.range.end_bound()
		{
			std::ops::Bound::Included(x) => x + 1,
			std::ops::Bound::Excluded(x) => *x,
			std::ops::Bound::Unbounded => s.as_ref().chars().count(),
		};
		let count_capped = s.as_ref().chars()
			.filter(|ch| ch.eq(&self.character))
			.take(max_cap)
			.count();
		self.range.contains(&count_capped)
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let pairs = self.input.lines()
			.map(|s|
			{
				let mut parts = s.split_whitespace();
				let range = parts.next().ok_or(Error::AocParsing)?;
				let range = range.split('-').collect::<Vec<_>>();
				if range.len() != 2 { bail!(Error::AocParsing); }
				let range = range[0].parse::<usize>()?..=range[1].parse()?;
				let ch = parts.next().ok_or(Error::AocParsing)?;
				let ch = ch.chars().next().ok_or(Error::AocParsing)?;
				let password = parts.next().ok_or(Error::AocParsing)?;
				if parts.next().is_some() { bail!(Error::AocParsing); }
				Ok((PasswordPolicy::from((range,ch)),password.to_string()))
			})
			.collect::<Result<Vec<(PasswordPolicy<_>,String)>>>()?;

		let num_valid = pairs.iter()
			.filter(|(policy,password)| policy.check(password))
			.count();

		Ok(format!("{}", num_valid))
	}
}

