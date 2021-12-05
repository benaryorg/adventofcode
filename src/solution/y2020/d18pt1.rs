use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D18Pt1 as Solution,
/// #     Solution as S,
/// # };
/// assert_eq!(Solution::new("2 * 3 + (4 * 5)".to_string()).solve().expect("1"), "26");
/// assert_eq!(Solution::new("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()).solve().expect("2"), "437");
/// assert_eq!(Solution::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()).solve().expect("3"), "12240");
/// assert_eq!(Solution::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()).solve().expect("4"), "13632");
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

fn number(input: &str) -> IResult<&str, isize>
{
	let (input, number) = digit1(input)?;
	match number.parse()
	{
		Ok(number) => Ok((input,number)),
		Err(_) => fail(input),
	}
}

fn calculable(input: &str) -> IResult<&str, isize>
{
	let (input, num) = alt((parenthesis, number))(input)?;
	fold_many0(calculation, move || num, |num, calc| calc.calculate(num))(input)
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

	Ok((input, calc))
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
			.collect::<Result<Vec<isize>>>()?
			.into_iter()
			.sum::<isize>();

		Ok(format!("{}", result))
	}
}

