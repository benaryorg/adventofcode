use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D16Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "class: 1-3 or 5-7\n\
///     row: 6-11 or 33-44\n\
///     seat: 13-40 or 45-50\n\
///     \n\
///     your ticket:\n\
///     7,1,14\n\
///     \n\
///     nearby tickets:\n\
///     7,3,47\n\
///     40,4,50\n\
///     55,2,20\n\
///     38,6,12";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "71");
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
#[derive(Clone,Debug)]
struct Rule
{
	ranges: Vec<std::ops::RangeInclusive<usize>>,
}

impl Rule
{
	fn validate(&self, number: usize) -> bool
	{
		self.ranges.iter().any(|range| range.contains(&number))
	}
}

impl std::str::FromStr for Rule
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let ranges = input.split(" or ")
			.map(|string|
			{
				string.split('-')
					.map(|i| Ok(i.parse()?))
					.collect::<Result<Vec<usize>>>()
					.and_then(|vec|
					{
						if vec.len() != 2
						{
							bail!(ErrorKind::ParseError);
						}
						Ok(vec[0]..=vec[1])
					})
			})
			.collect::<Result<_>>()?;
		Ok(Self { ranges, })
	}
}

struct Ticket(Vec<usize>);

impl std::str::FromStr for Ticket
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		Ok(Ticket(input.split(',').map(|i| Ok(i.parse()?)).collect::<Result<_>>()?))
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut parts = self.input.split("\n\n");

		let rules = parts.next().ok_or(ErrorKind::ParseError)?.lines()
			.map(|line|
			{
				let mut split = line.splitn(2,": ");
				let name = split.next().ok_or(ErrorKind::ParseError)?;
				let rule = split.next().ok_or(ErrorKind::ParseError)?.parse::<Rule>()?;
				Ok((name,rule))
			})
			.collect::<Result<std::collections::HashMap<_,_>>>()?;

		let _my_ticket = parts.next().ok_or(ErrorKind::ParseError)?.lines().nth(1).ok_or(ErrorKind::ParseError)?;

		let tickets = parts.next().ok_or(ErrorKind::ParseError)?.lines().skip(1)
			.map(|line| Ok(line.parse::<Ticket>()?))
			.collect::<Result<Vec<Ticket>>>()?;

		let sum: usize = tickets.iter()
			.flat_map(|ticket| ticket.0.iter().copied().filter(|&num| !rules.values().any(|rule| rule.validate(num))))
			.sum();

		Ok(format!("{}", sum))
	}
}

