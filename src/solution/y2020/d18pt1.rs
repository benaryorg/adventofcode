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
	alt,
	char,
	delimited,
	flat_map,
	fold_many0,
	named,
	parse_to,
	switch,
	tuple,
	character::streaming::digit1,
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

named!(number<isize>, flat_map!(digit1, parse_to!(isize)));
named!(calculable<isize>, switch!(alt!(number | parenthesis), num => fold_many0!(calculation, num, |num,calc| calc.calculate(num))));

named!(calculation<Calculation>, alt!
(
	tuple!(char!('*'),alt!(number | parenthesis)) => { |(_,num)| Calculation::Multiply(num) } |
	tuple!(char!('+'),alt!(number | parenthesis)) => { |(_,num)| Calculation::Add(num) }
));

named!(parenthesis<isize>, delimited!(char!('('), calculable, char!(')')));

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let result = self.input.lines()
			.map(|line|
			{
				format!("({})", line).bytes()
					.filter(|b| !b.is_ascii_whitespace())
					.collect::<Vec<u8>>()
			})
			.map(|slice| Ok(parenthesis(&slice).map_err(|_| ErrorKind::ParseError)?.1))
			.collect::<Result<Vec<isize>>>()?
			.into_iter()
			.sum::<isize>();

		Ok(format!("{}", result))
	}
}

