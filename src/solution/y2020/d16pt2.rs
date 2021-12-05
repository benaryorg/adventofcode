use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D16Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "class: 0-1 or 4-19\n\
///     row: 0-5 or 8-19\n\
///     seat: 0-13 or 16-19\n\
///     \n\
///     your ticket:\n\
///     11,12,13\n\
///     \n\
///     nearby tickets:\n\
///     3,9,18\n\
///     15,1,5\n\
///     5,14,9";
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
	fn from_str(input: &str) -> std::result::Result<Self, Error>
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
							bail!(Error::AocParsing);
						}
						Ok(vec[0]..=vec[1])
					})
			})
			.collect::<Result<_>>().context("parsing for rule")?;
		Ok(Self { ranges, })
	}
}

struct Ticket(Vec<usize>);

impl std::str::FromStr for Ticket
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		Ok(Ticket(input.split(',').map(|i| Ok(i.parse()?)).collect::<Result<_>>()?))
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut parts = self.input.split("\n\n");

		let rules = parts.next().ok_or(Error::AocParsing)?.lines()
			.map(|line|
			{
				let mut split = line.splitn(2,": ");
				let name = split.next().ok_or(Error::AocParsing)?;
				let rule = split.next().ok_or(Error::AocParsing)?.parse::<Rule>()?;
				Ok((name,rule))
			})
			.collect::<Result<std::collections::HashMap<_,_>>>()?;

		let my_ticket = parts.next().ok_or(Error::AocParsing)?.lines().nth(1).ok_or(Error::AocParsing)?.parse::<Ticket>()?;

		let tickets = parts.next().ok_or(Error::AocParsing)?.lines().skip(1)
			.map(|line| Ok(line.parse::<Ticket>()?))
			.collect::<Result<Vec<Ticket>>>()?
			.into_iter()
			.filter(|ticket| ticket.0.iter().all(|&num| rules.values().any(|rule| rule.validate(num))))
			.collect::<Vec<_>>();

		let mut possibilities = rules.iter()
			.flat_map(|kv|
			{
				(0..rules.len())
					.map(
					{
						let kv = kv;
						move |i| (i,kv)
					})
			})
			.filter(|&(idx,(_,rule))| tickets.iter().all(|ticket| ticket.0.get(idx).map(|&value| rule.validate(value)).unwrap_or(false)))
			.map(|(idx,(name,_))| (name,idx))
			.collect::<Vec<_>>();

		let mut mapping = std::collections::HashMap::new();

		loop
		{
			if possibilities.is_empty()
			{
				break;
			}
			let map = possibilities.iter().map(|(_,idx)| idx).copied()
				.fold(std::collections::BTreeMap::<usize,usize>::new(), |mut map, value|
				{
					map.entry(value)
						.and_modify(|value| *value += 1)
						.or_insert(1);
					map
				});
			for (key,_) in map.into_iter().filter(|&(_,value)| value == 1)
			{
				let (name,_) = possibilities.iter().cloned().find(|(_,idx)| *idx == key).unwrap();
				mapping.insert(name, key);
				possibilities.retain(|e| e.1 != key && e.0 != name);

				println!("{}: {}", name, key);
			}
		}

		Ok(format!("{}", mapping.into_iter().filter(|(name,_)| name.starts_with("departure")).map(|(_,idx)| Ok(my_ticket.0.get(idx).ok_or(Error::AocParsing)?)).collect::<Result<Vec<_>>>()?.into_iter().product::<usize>()))
	}
}

