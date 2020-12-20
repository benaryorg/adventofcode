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
	alt,
	char,
	delimited,
	flat_map,
	many1,
	map,
	named,
	parse_to,
	tuple,
	character::streaming::digit1,
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

named!(number<isize>, flat_map!(digit1, parse_to!(isize)));
named!(calculable<isize>, map!(tuple!(alt!(number | parenthesis), many1!(calculation)), |(mut num, mut calcs)|
{
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
	calcs.into_iter().fold(num, |acc, calc| calc.calculate(acc))
}));

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

