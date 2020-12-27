use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D2Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "1-3 a: abcde\n\
///     1-3 b: cdefg\n\
///     2-9 c: ccccccccc";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "1");
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
struct PasswordPolicy
{
	first: usize,
	second: usize,
	character: char,
}

impl From<((usize,usize), char)> for PasswordPolicy
{
	fn from(other: ((usize, usize), char)) -> Self
	{
		PasswordPolicy
		{
			first: other.0.0 - 1,
			second: other.0.1 - 1,
			character: other.1,
		}
	}
}

impl PasswordPolicy
{
	fn check<S: AsRef<str>>(&self, s: S) -> bool
	{
		let chars = s.as_ref().chars().collect::<Vec<char>>();
		let first = chars.get(self.first).map(|&ch| ch == self.character).unwrap_or(false);
		let second = chars.get(self.second).map(|&ch| ch == self.character).unwrap_or(false);

		first ^ second
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
				let chars = parts.next().ok_or(ErrorKind::ParseError)?;
				let chars = chars.split('-').collect::<Vec<_>>();
				if chars.len() != 2 { bail!(ErrorKind::ParseError); }
				let chars = (chars[0].parse::<usize>()?, chars[1].parse()?);
				let ch = parts.next().ok_or(ErrorKind::ParseError)?;
				let ch = ch.chars().next().ok_or(ErrorKind::ParseError)?;
				let password = parts.next().ok_or(ErrorKind::ParseError)?;
				if parts.next() != None { bail!(ErrorKind::ParseError); }
				Ok((PasswordPolicy::from((chars,ch)),password.to_string()))
			})
			.collect::<Result<Vec<(PasswordPolicy,String)>>>()?;

		let num_valid = pairs.iter()
			.filter(|(policy,password)| policy.check(password))
			.count();

		Ok(format!("{}", num_valid))
	}
}

