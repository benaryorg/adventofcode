use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D18Pt2 as Solution,
/// #     Solution as S,
/// # };
/// assert_eq!(Solution::new("1 + (2 * 3) + (4 * (5 + 6))".to_string()).solve().expect("1"), "51");
/// assert_eq!(Solution::new("2 * 3 + (4 * 5)".to_string()).solve().expect("2"), "46");
/// assert_eq!(Solution::new("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()).solve().expect("3"), "1445");
/// assert_eq!(Solution::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()).solve().expect("4"), "669060");
/// assert_eq!(Solution::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 ".to_string()).solve().expect("4"), "23340");
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

use nom::
{
	character::complete::*,
	combinator::*,
	branch::*,
	multi::*,
	IResult,
};

#[derive(Clone,Debug)]
enum Calculation
{
	Add(isize),
	Multiply(isize),
}

impl Calculation
{
	fn calculate(self, lhs: isize) -> isize
	{
		match self
		{
			Calculation::Add(rhs) => lhs + rhs,
			Calculation::Multiply(rhs) => lhs * rhs,
		}
	}
}

fn calculable(input: &str) -> IResult<&str, isize>
{
	let (input, mut num) = alt((number, parenthesis))(input)?;
	let (input, mut calcs) = many1(calculation)(input)?;

	for i in (0..calcs.len()).rev()
	{
		if let Calculation::Add(x) = calcs[i].clone()
		{
			calcs.swap_remove(i);
			if i == 0
			{
				num += x;
			}
			else
			{
				match calcs[i-1]
				{
					Calculation::Add(ref mut i) => *i += x,
					Calculation::Multiply(ref mut i) => *i += x,
				}
			}
		}
	}

	let res = calcs.into_iter().fold(num, |acc, calc| calc.calculate(acc));
	Ok((input, res))
}

fn number(input: &str) -> IResult<&str, isize>
{
	let (input, number) = digit1(input)?;
	match number.parse()
	{
		Ok(number) => Ok((input,number)),
		Err(_) => fail(input),
	}
}

fn calculation(input: &str) -> IResult<&str, Calculation>
{
	let (input, op) = one_of("*+")(input)?;
	let (input, num) = alt((number, parenthesis))(input)?;

	match op
	{
		'*' => Ok((input, Calculation::Multiply(num))),
		'+' => Ok((input, Calculation::Add(num))),
		_ => unreachable!(),
	}
}

fn parenthesis(input: &str) -> IResult<&str, isize>
{
	let (input, _) = char('(')(input)?;
	let (input, calc) = calculable(input)?;
	let (input, _) = char(')')(input)?;

	return Ok((input, calc));
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let result = self.input.lines()
			.map(|line|
			{
				format!("({})", line).chars()
					.filter(|ch| !ch.is_ascii_whitespace())
					.collect::<String>()
			})
			.map(|slice| Ok(parenthesis(slice.as_str()).map_err(|_| Error::AocParseError)?.1))
			.collect::<std::result::Result<Vec<isize>, Error>>()?
			.into_iter()
			.sum::<isize>();

		Ok(format!("{}", result))
	}
}

