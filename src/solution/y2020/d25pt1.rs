use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D25Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "5764801\n17807724";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "14897079");
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

struct Crypto
{
	value: usize,
	subject: usize,
}

impl Iterator for Crypto
{
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item>
	{
		let ret = Some(self.value);
		self.value = (self.value * self.subject) % 20201227;
		ret
	}
}

fn transform(subject: usize) -> Crypto
{
	Crypto
	{
		subject,
		value: 1,
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let mut lines = self.input.lines();
		const SUBJECT: usize = 7;
		let card_pub = lines.next().ok_or(Error::AocParsing)?.parse()?;
		let door_pub = lines.next().ok_or(Error::AocParsing)?.parse()?;
		if lines.next().is_some()
		{
			bail!(Error::AocParsing);
		}

		let card_loop = transform(SUBJECT).enumerate().find(|&(_,value)| value == card_pub).ok_or(Error::AocNoSolution)?.0;
		let door_loop = transform(SUBJECT).enumerate().find(|&(_,value)| value == door_pub).ok_or(Error::AocNoSolution)?.0;

		let card_enc_key = transform(door_pub).nth(card_loop).ok_or(Error::AocNoSolution)?;
		let door_enc_key = transform(card_pub).nth(door_loop).ok_or(Error::AocNoSolution)?;

		if card_enc_key != door_enc_key
		{
			bail!(Error::AocNoSolution);
		}

		let enc_key = card_enc_key;

		Ok(format!("{}", enc_key))
	}
}

