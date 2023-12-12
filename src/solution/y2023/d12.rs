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

fn validate<I>(iter: I, groups: &[usize]) -> Option<bool>
	where I: Iterator<Item=u8>
{
	let mut iter = iter.peekable();
	let mut sum = groups.iter().sum::<usize>();
	let mut groups = groups.iter().copied().peekable();
	let mut cur = 0;

	while let Some(ch) = iter.next()
	{
		match ch
		{
			b'#' =>
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
			b'.' =>
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
			b'?' =>
			{
				if groups.next().is_some()
				{
					let remaining = iter.filter(|&b| b == b'#' || b == b'?').count();
					if remaining + 1 < sum
					{
						return Some(false);
					}
					return None;
				}
				else
				{
					if cur > 0 || iter.any(|b| b == b'#')
					{
						return Some(false);
					}
					else
					{
						return Some(true);
					}
				}
			},
			_ => unreachable!(),
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut stack = self.input.lines()
			.map(|line|
			{
				let (map, groups) = line.split_once(' ').ok_or(Error::AocParsing)?;
				let map = map.to_string();
				let groups = groups.split(',').map(|s| Ok(s.parse::<usize>()?)).collect::<Result<Vec<_>>>()?;
				match self.folded
				{
					true =>
					{
						let map = std::iter::repeat(map)
							.take(5)
							.fold(String::new(), |mut acc, s|
							{
								if !acc.is_empty()
								{
									acc.push('?');
								}
								acc.extend(s.chars());
								acc
							});
						let groups = std::iter::repeat(groups)
							.take(5)
							.flatten()
							.collect::<Vec<_>>();
						Ok((map.into_bytes(), groups))
					},
					false => Ok((map.into_bytes(), groups)),
				}
			})
			.collect::<Result<Vec<_>>>()?;

		let result: usize = std::iter::from_fn(||
			{
				while let Some((mut map, groups)) = stack.pop()
				{
					trace!("input: {:?} ({:?})", map, groups);
					if let Some(idx) = map.iter().position(|&b| b == b'?')
					{
						[b'.', b'#'].iter()
							.for_each(|&ch|
							{
								map[idx] = ch;
								trace!("trying {:?}", map);
								if validate(map.iter().copied(), &groups) != Some(false)
								{
									stack.push((map.clone(), groups.clone()));
								}
							});
					}
					else
					{
						return Some((map, groups));
					}
				}
				None
			})
			.inspect(|(map, groups)| debug!("ok: {:?} ({:?})", String::from_utf8_lossy(map), groups))
			.count();

		Ok(format!("{}", result))
	}
}

