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

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d14pt1::Solution, Solution as S };
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "1588");
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

fn full_input(input: &str) -> IResult<&str, (&str, std::collections::BTreeMap<(char, char), char>)>
{
	let (input, chars) = terminated(alpha1, newline)(input)?;
	let (input, _) = char('\n')(input)?;
	let (input, lines) = many1(line)(input)?;
	let (input, _) = eof(input)?;
	Ok((input, (chars, lines.into_iter().collect())))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);
		let (_, (input, translation)) = terminated(full_input, eof)(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let output = std::iter::successors(Some(input.chars().collect::<Vec<char>>()),|input|
		{
			Some(input.windows(2)
				.fold(vec![input[0]], |mut vec, win|
				{
					let a = win[0];
					let b = win[1];
					if let Some(&mid) = translation.get(&(a, b))
					{
						vec.push(mid);
					}
					vec.push(b);
					vec
				}))
		})
			.nth(10).ok_or(Error::AocNoSolution)?;

		let counts = output.into_iter().fold(std::collections::BTreeMap::new(), |mut map, ch|
		{
			*map.entry(ch).or_insert(0) += 1;
			map
		}).into_iter().map(|(_, num)| num).collect::<Vec<_>>();

		let result = counts.iter().max().ok_or(Error::AocNoSolution)? - counts.iter().min().ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", result))
	}
}

