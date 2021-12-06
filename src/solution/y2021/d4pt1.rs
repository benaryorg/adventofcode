use crate::error::*;

use std::convert::TryInto;
use std::str::FromStr;

use nom::
{
	character::complete::*,
	number::complete::*,
	sequence::*,
	multi::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d4pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
///     \n\
///     22 13 17 11  0\n\
///      8  2 23  4 24\n\
///     21  9 14 16  7\n\
///      6 10  3 18  5\n\
///      1 12 20 15 19\n\
///     \n\
///      3 15  0  2 22\n\
///      9 18 13 17  5\n\
///     19  8  7 25 23\n\
///     20 11 10 24  4\n\
///     14 21 16 12  6\n\
///     \n\
///     14 21 17 24  4\n\
///     10 16 15  9 19\n\
///     18  8 23 26 20\n\
///     22 11 13  6  5\n\
///      2  0 12  3  7\n";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "4512");
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

#[derive(Debug,Hash,Eq,PartialEq)]
struct Bingo
{
	numbers: [usize;25],
}

impl Bingo
{
	fn check(&self, numbers: &std::collections::BTreeSet<usize>) -> Option<usize>
	{
		let chunks = self.numbers.chunks(5).collect::<Vec<_>>();
		let got_row = chunks.iter().any(|chunk| chunk.iter().all(|n| numbers.contains(n)));
		let got_col = chunks[0].iter().zip(chunks[1]).zip(chunks[2]).zip(chunks[3]).zip(chunks[4])
			.any(|((((n1,n2),n3),n4),n5)| [n1,n2,n3,n4,n5].iter().all(|n| numbers.contains(n)));

		debug!("{:?}", chunks[0]);

		if !got_row && !got_col
		{
			return None;
		}
		Some(self.numbers.iter().filter(|n| !numbers.contains(n)).sum())
	}
}

impl From<[usize; 25]> for Bingo
{
	fn from(input: [usize; 25]) -> Self
	{
		Self
		{
			numbers: input,
		}
	}
}

impl FromStr for Bingo
{
	type Err = anyhow::Error;

	fn from_str(input: &str) -> Result<Self>
	{
		Ok(Self
		{
			numbers: parse_bingo(input).map_err(|_| Error::AocParsing)?.1,
		})
	}
}

fn parse_bingos(input: &str) -> IResult<&str, Vec<[usize; 25]>>
{
	separated_list1(char('\n'),parse_bingo)(input)
}

fn parse_bingo(input: &str) -> IResult<&str, [usize; 25]>
{
	let (input, lines) = count(terminated(parse_bingo_line, char('\n')), 5)(input)?;

	let numbers = lines.into_iter().flatten().collect::<Vec<_>>();

	Ok((input, numbers.try_into().unwrap()))
}

fn parse_bingo_line(input: &str) -> IResult<&str, [usize;5]>
{
	let (input, (_, num1, _, num2, _, num3, _, num4, _, num5)) = tuple((space0, double, space1, double, space1, double, space1, double, space1, double))(input)?;

	Ok((input,[num1 as usize, num2 as usize, num3 as usize, num4 as usize, num5 as usize]))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut parts = self.input.splitn(2, "\n\n");
		let numbers = parts.next().ok_or(Error::AocParsing).context("could not parse numbers")?
			.split(',')
			.map(|s| Ok(s.parse()?))
			.collect::<Result<Vec<usize>>>()?;

		let rest = parts.next().ok_or(Error::AocParsing).context("no bingos found")?;
		let (rest, vec) = parse_bingos(rest).map_err(|e| anyhow!("{}", e)).context("could not parse bingos")?;
		if !rest.is_empty()
		{
			return Err(Error::AocParsing).context(anyhow!("incomplete parsing, remainder '{}'", rest));
		}
		let bingos = vec.into_iter().map(Bingo::from).collect::<Vec<_>>();

		let mut set: std::collections::BTreeSet<usize> = Default::default();

		for number in numbers
		{
			debug!("add to set: {}", number);
			set.insert(number);

			for bingo in &bingos
			{
				if let Some(result) = bingo.check(&set)
				{
					return Ok(format!("{}", number*result));
				}
			}
		}

		bail!(Error::AocNoSolution);
	}
}

