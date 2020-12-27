use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D14Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
///     mem[8] = 11\n\
///     mem[7] = 101\n\
///     mem[8] = 0";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "165");
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

struct Mask
{
	zero: u64,
	one: u64,
}

impl Mask
{
	fn apply(&self, num: u64) -> u64
	{
		(num | self.one) & !self.zero
	}
}

impl std::str::FromStr for Mask
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let mut zero = 0;
		let mut one = 0;

		for ch in input.chars()
		{
			zero <<= 1;
			one <<= 1;
			match ch
			{
				'1' => one |= 1,
				'0' => zero |= 1,
				'X' => {},
				_ => bail!(ErrorKind::ParseError),
			}
		}

		Ok(Mask { zero, one, })
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let result = self.input.lines()
			.fold((Default::default(),"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse::<Mask>()?),|(mut acc,mask): (std::collections::BTreeMap<usize,u64>,Mask), next|
			{
				if next.starts_with("mask =")
				{
					(acc, next.split("=").nth(1).ok_or(ErrorKind::ParseError).unwrap().trim().parse().unwrap())
				}
				else
				{
					lazy_static::lazy_static!
					{
						static ref RE: regex::Regex = regex::Regex::new(r"\Amem\[(?P<key>\d+)\] = (?P<value>\d+)\z").unwrap();
					}
					let captures = RE.captures(next).ok_or(ErrorKind::ParseError).unwrap();
					acc.insert(captures.name("key").unwrap().as_str().parse().unwrap(),mask.apply(captures.name("value").unwrap().as_str().parse().unwrap()));
					(acc,mask)
				}
			}).0;

		Ok(format!("{:?}", result.values().sum::<u64>()))
	}
}

