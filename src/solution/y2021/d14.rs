use crate::error::*;

use nom::
{
	character::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
};

// FIXME: this is broken.
// The unit tests pass, but everything else is broken.
// To quote someone looking over my numbers:
//
// > basically almost all (except one, one is correct) of the numbers are off by at least one (but also single digit)
// > yes, all (except the one that is correct) are too high
//
// And my personal favourite:
// 
// > off-by-more-than-one-but-single-digit how did you do that

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d14::Solution, Solution as S };
/// # env_logger::init();
/// let input = "NNCB\n\
///     \n\
///     CH -> B\n\
///     HH -> N\n\
///     CB -> H\n\
///     NH -> C\n\
///     HB -> C\n\
///     HC -> B\n\
///     HN -> C\n\
///     NN -> C\n\
///     BH -> H\n\
///     NC -> B\n\
///     NB -> B\n\
///     BN -> B\n\
///     BB -> N\n\
///     BC -> B\n\
///     CC -> N\n\
///     CN -> C\n";
/// assert_eq!(Solution::with_steps(10, input.to_string()).solve().unwrap(), "1588");
/// assert_eq!(Solution::with_steps(40, input.to_string()).solve().unwrap(), "2188189693529");
/// ```
pub struct Solution
{
	input: String,
	steps: usize,
}

impl Solution
{
	pub fn with_steps(steps: usize, input: String) -> Self
	{
		Self
		{
			input,
			steps,
		}
	}
}

fn line(input: &str) -> IResult<&str, ((char, char), char)>
{
	terminated(
		separated_pair(
			tuple(
				( one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
				, one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
				)
			),
			tag(" -> "),
			one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
		),
		newline
	)(input)
}

fn full_input(input: &str) -> IResult<&str, (&str, std::collections::HashMap<(char, char), char>)>
{
	let (input, chars) = terminated(alpha1, newline)(input)?;
	let (input, _) = char('\n')(input)?;
	let (input, lines) = many1(line)(input)?;
	let (input, _) = eof(input)?;
	Ok((input, (chars, lines.into_iter().collect())))
}

fn unfold(lookup: &std::collections::HashMap<(char, char), char>, a: char, b: char, depth_limit: usize, counter: &mut std::collections::HashMap<(char, char, usize, char), u128>)
{
	for ch in 'A'..='Z'
	{
		if depth_limit == 0
		{
			let count = ((a == ch) as u128) + ((b == ch) as u128);
			trace!("got nothing: {},{}: {} for {}", a, b, count, ch); 
			counter.insert((a, b, depth_limit, ch), count);
			continue;
		}
		if !counter.contains_key(&(a, b, depth_limit, ch))
		{
			if let Some(&mid) = lookup.get(&(a, b))
			{
				unfold(lookup, a, mid, depth_limit-1, counter);
				unfold(lookup, mid, b, depth_limit-1, counter);
				let count_a = counter.get(&(a, mid, depth_limit-1, ch)).copied().unwrap_or(0);
				let count_b = counter.get(&(mid, b, depth_limit-1, ch)).copied().unwrap_or(0);
				let count = (count_a + count_b)
					.saturating_sub(if mid == ch { 1 } else { 0 });
				trace!("{:?}: {}", (a, b, depth_limit, ch), count);
				counter.insert((a, b, depth_limit, ch), count);
			}
			else
			{
				let count = ((a == ch) as u128) + ((b == ch) as u128);
				trace!("got nothing: {},{}: {} for {}", a, b, count, ch); 
				counter.insert((a, b, depth_limit, ch), count);
			}
		}
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);
		let (_, (input, lookup)) = terminated(full_input, eof)(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let mut counts: std::collections::HashMap::<(char, char, usize, char), u128> = Default::default();

		let counts = input.chars().collect::<Vec<char>>().windows(2)
			.flat_map(|v|
			{
				let a = v[0];
				let b = v[1];
				debug!("running pair: ({}, {})", a, b);

				unfold(&lookup, a, b, self.steps, &mut counts);
				('A'..='Z').map(|ch| (ch, counts.get(&(a, b, self.steps, ch)).copied().unwrap_or(0))).collect::<Vec<_>>()
			})
			.fold(std::collections::HashMap::<char, u128>::new(), |mut map, (ch, count)|
			{
				*map.entry(ch).or_insert(0) += count;
				map
			});

		debug!("{:?}", counts);

		let nums = counts
			.into_iter()
			.filter(|&(_, num)| num != 0)
			.map(|(_, num)| num)
			.collect::<Vec<_>>();

		let max = nums.iter()
			.max()
			.ok_or(Error::AocNoSolution)?;
		let min = nums.iter()
			.min()
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", max-min))
	}
}

