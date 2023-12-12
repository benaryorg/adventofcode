use crate::error::*;

/// # Examples
///
/// Part 1:
///
/// ```
/// # use adventofcode::solution::{ y2023::d12::Solution, Solution as S };
/// # env_logger::init();
/// let input = "???.### 1,1,3";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "1");
/// let input = ".??..??...?##. 1,1,3";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "4");
/// let input = "?#?#?#?#?#?#?#? 1,3,1,6";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "1");
/// let input = "????.#...#... 4,1,1";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "1");
/// let input = "????.######..#####. 1,6,5";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "4");
/// let input = "?###???????? 3,2,1";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "10");
/// let input = "\
///    ???.### 1,1,3\n\
///    .??..??...?##. 1,1,3\n\
///    ?#?#?#?#?#?#?#? 1,3,1,6\n\
///    ????.#...#... 4,1,1\n\
///    ????.######..#####. 1,6,5\n\
///    ?###???????? 3,2,1";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "21");
/// ```
///
/// Part 2:
///
/// ```
/// # use adventofcode::solution::{ y2023::d12::Solution, Solution as S };
/// # env_logger::init();
/// let input = "???.### 1,1,3";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "1");
/// let input = ".??..??...?##. 1,1,3";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "16384");
/// let input = "?#?#?#?#?#?#?#? 1,3,1,6";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "1");
/// let input = "????.#...#... 4,1,1";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "16");
/// let input = "????.######..#####. 1,6,5";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "2500");
/// let input = "?###???????? 3,2,1";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "506250");
/// let input = "\
///    ???.### 1,1,3\n\
///    .??..??...?##. 1,1,3\n\
///    ?#?#?#?#?#?#?#? 1,3,1,6\n\
///    ????.#...#... 4,1,1\n\
///    ????.######..#####. 1,6,5\n\
///    ?###???????? 3,2,1";
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "525152");
/// ```
pub struct Solution
{
	input: String,
	folded: bool,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			folded: false,
			input,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			folded: true,
			input,
		}
	}
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Spring
{
	Good,
	Bad,
	Unknown,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Row
{
	springs: Vec<Spring>,
	groups: Vec<usize>,
}

fn validate(row: &[Spring], groups: &[usize]) -> Option<bool>
{
	let mut iter = row.iter().peekable();
	let mut sum = groups.iter().sum::<usize>();
	let mut groups = groups.iter().copied().peekable();
	let mut cur = 0;

	while let Some(ch) = iter.next()
	{
		match ch
		{
			Spring::Bad =>
			{
				if sum == 0
				{
					return Some(false);
				}
				else
				{
					sum -= 1;
					cur += 1;
					if let Some(&next) = groups.peek()
					{
						if cur > next
						{
							return Some(false);
						}
					}
					else
					{
						return Some(false);
					}
				}
			},
			Spring::Good =>
			{
				if cur > 0
				{
					if let Some(group) = groups.next()
					{
						if cur != group
						{
							return Some(false);
						}
						cur = 0;
					}
					else
					{
						return Some(false);
					}
				}
			},
			Spring::Unknown =>
			{
				if groups.next().is_some()
				{
					let remaining = iter.filter(|&&b| b == Spring::Bad || b == Spring::Unknown).count();
					if remaining + 1 < sum
					{
						return Some(false);
					}
					return None;
				}
				else
				{
					if cur > 0 || iter.any(|&b| b == Spring::Bad)
					{
						return Some(false);
					}
					else
					{
						return Some(true);
					}
				}
			},
		}
	}
	if let Some(group) = groups.next()
	{
		if cur != group || groups.next().is_some()
		{
			return Some(false);
		}
	}
	else
	{
		if cur != 0
		{
			return Some(false);
		}
	}
	if sum != 0
	{
		return Some(false);
	}
	Some(true)
}

impl Row
{
	fn permutations<'a>(map: &mut std::collections::HashMap<(Vec<Spring>, &'a [usize]), u128>, row: &[Spring], groups: &'a [usize]) -> u128
	{
		map.get(&(row.to_vec(), groups))
			.copied()
			.unwrap_or_else(||
			{
				let num = match validate(&row, groups)
				{
					Some(false) => 0,
					Some(true) => 1,
					None =>
					{
						let (mut row, groups) =
						{
							let (next, restg) = groups.split_first().unwrap();
							let prefix = row.iter()
								.copied()
								.take_while(|&s| s == Spring::Good)
								.chain(std::iter::repeat(Spring::Bad).take(*next))
								.chain(std::iter::once(Spring::Good))
								.collect::<Vec<_>>();
							if let Some(new_row) = row.strip_prefix(&prefix[..])
							{
								(new_row.to_vec(), restg)
							}
							else
							{
								(row.to_vec(), groups)
							}
						};
						let position = row.iter().position(|&s| s == Spring::Unknown).unwrap();
						row[position] = Spring::Good;
						let good = Row::permutations(map, &row, &groups);
						row[position] = Spring::Bad;
						let bad = Row::permutations(map, &row, &groups);
						row[position] = Spring::Unknown;
						map.insert((row, groups), good + bad);
						good + bad
					},
				};
				num
			})
	}

	fn solve(&self) -> u128
	{
		Row::permutations(&mut Default::default(), &self.springs, &self.groups)
	}
}

impl std::str::FromStr for Row
{
	type Err = Error;
	fn from_str(line: &str) -> std::result::Result<Self, Error>
	{
		let (springs, groups) = line.split_once(' ').ok_or(Error::AocParsing)?;
		let springs = springs.chars()
			.map(|spring|
			{
				match spring
				{
					'.' => Ok(Spring::Good),
					'#' => Ok(Spring::Bad),
					'?' => Ok(Spring::Unknown),
					ch => Err(anyhow!("unknown spring char {:?}", ch)).context(Error::AocParsing),
				}
			})
			.collect::<Result<Vec<_>>>()?;
		let groups = groups.split(',')
			.map(|group| Ok(group.parse::<usize>()?))
			.collect::<Result<Vec<_>>>()?;

		Ok(Self
		{
			springs,
			groups,
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let rows = self.input.lines()
			.map(|line|
			{
				if !self.folded
				{
					Ok(line.to_string())
				}
				else
				{
					let (map, groups) = line.split_once(' ').ok_or(Error::AocParsing)?;
					let map = [map].repeat(5).join("?");
					let groups = [groups].repeat(5).join(",");
					Ok([map, groups].join(" "))
				}
			})
			.map(|res| res.and_then(|line| Ok(line.parse::<Row>()?)))
			.collect::<Result<Vec<_>>>()?;

		trace!("rows:\n{:#?}", rows);

		let result: u128 = rows.iter()
			.enumerate()
			.map(|(idx, row)|
			{
				let num = row.solve();
				debug!("row {}: {}", idx, num);
				num
			})
			.sum();

		Ok(format!("{}", result))
	}
}

