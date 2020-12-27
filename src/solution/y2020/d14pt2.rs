use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D14Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "mask = 000000000000000000000000000000X1001X\n\
///     mem[42] = 100\n\
///     mask = 00000000000000000000000000000000X0XX\n\
///     mem[26] = 1";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "208");
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
	fn apply(&self, num: u64) -> Vec<u64>
	{
		let base = num | self.one;
		let floatmask = !(self.one | self.zero) & 0x0000_000f_ffff_ffff;
		let count = (1 << floatmask.count_ones()) >> 1 as usize;
		std::iter::repeat(base)
			.take(count*2)
			.enumerate()
			.map(|(idx,mut num)|
			{
				let idx = idx as u64;
				let mut mask_offset = 0;
				for idx_offset in 0..floatmask.count_ones()
				{
					while (1 << (idx_offset + mask_offset)) & floatmask == 0
					{
						mask_offset += 1;
					}
					num = (num & !(1 << (idx_offset + mask_offset))) | (idx & (1 << idx_offset)) << mask_offset;
				}
				num
			})
			.collect()
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
			.fold((Default::default(),"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse::<Mask>()?),|(mut acc,mask): (std::collections::BTreeMap<u64,u64>,Mask), next|
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
					let key = captures.name("key").unwrap().as_str().parse().unwrap();
					let value = captures.name("value").unwrap().as_str().parse().unwrap();
					for key in mask.apply(key)
					{
						acc.insert(key, value);
					}
					(acc,mask)
				}
			}).0;

		Ok(format!("{:?}", result.values().sum::<u64>()))
	}
}

