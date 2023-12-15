use crate::error::*;

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d15::Solution, Solution as S };
/// # env_logger::init();
/// let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "1320");
/// ```
///
/// Part 2:
///
/// ```
/// # use adventofcode::solution::{ y2023::d15::Solution, Solution as S };
/// # env_logger::init();
/// let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "145");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self { part: Part::One, input, }
	}

	pub fn part2(input: String) -> Self
	{
		Self { part: Part::Two, input, }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Part
{
	One,
	Two,
}

#[derive(Debug, Clone)]
enum Op
{
	Set((String, usize)),
	Remove(String),
}

impl Op
{
	fn full_hash(&self) -> u8
	{
		let input = match &self
		{
			Op::Set((s, l)) => format!("{}={}", s, l),
			Op::Remove(s) => format!("{}-", s),
		};
		input.bytes().fold(0usize, |cur, next| ((cur + usize::from(next)) * 17) % 256) as u8
	}

	fn hash(&self) -> u8
	{
		let input = match &self
		{
			Op::Set((s, _)) => s,
			Op::Remove(s) => s,
		};
		input.bytes().fold(0usize, |cur, next| ((cur + usize::from(next)) * 17) % 256) as u8
	}
}

impl std::str::FromStr for Op
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		if let Some((head, tail)) = input.split_once('=')
		{
			Ok(Op::Set((head.to_string(), tail.parse()?)))
		}
		else
		{
			Ok(Op::Remove(input.chars().take(input.len() - 1).collect()))
		}
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let ops = self.input.trim().split(',')
			.map(|s|
			{
				let op = s.parse::<Op>()?;
				trace!("op: {:?}", op);
				Ok(op)
			})
			.collect::<Result<Vec<Op>>>()?;

		let result: usize = match self.part
		{
			Part::One => ops.iter().map(Op::full_hash).map(usize::from).sum(),
			Part::Two =>
			{
				let data = std::collections::BTreeMap::<u8, Vec<(String, usize)>>::new();
				ops.into_iter()
					.fold(data, |mut data, op|
					{
						let hash = op.hash();
						match op
						{
							Op::Remove(opname) => data.entry(hash).or_default().retain(|(name, _)| !opname.eq(name)),
							Op::Set((opname, oplen)) =>
							{
								let entry = data.entry(hash).or_default();
								if !entry.iter_mut()
									.any(|(name, ref mut len)|
									{
										if !opname.eq(name)
										{
											false
										}
										else
										{
											*len = oplen;
											true
										}
									})
								{
									entry.push((opname, oplen));
								}
							},
						}
						trace!("state:\n{:?}", data);
						data
					})
					.into_iter()
					.flat_map(|(idx, bx)|
					{
						let bx_num = idx as usize + 1;
						bx.into_iter()
							.enumerate()
							.map(move |(idx, (_, len))| bx_num * (idx + 1) * len)
					})
					.sum()
			},
		};
		
		Ok(format!("{}", result))
	}
}

