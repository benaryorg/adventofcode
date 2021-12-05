use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D5Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "FBFBBFFRLR\n\
///     FBFBBFFRLL\n\
///     FBFBBFFLRR\n\
///     FBFBBFFLLR\n\
///     FBFBBFFLLL";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "354");
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
#[derive(Clone,Debug)]
struct Seat
{
	row: usize,
	column: usize,
}

impl Seat
{
	fn id(&self) -> usize
	{
		self.column + self.row * 8
	}
}

impl std::str::FromStr for Seat
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		lazy_static::lazy_static! {
			static ref RE: regex::Regex = regex::Regex::new(r"\A(?P<row>[FB]{7})(?P<column>[LR]{3})\z").unwrap();
		}
		let captures = RE.captures(input).ok_or(Error::AocParsing)?;
		let row = captures.name("row").unwrap().as_str()
			.chars()
			.rev()
			.enumerate()
			.map(|(idx,ch)| if ch == 'B' { 1 << idx } else { 0 })
			.sum();
		let column = captures.name("column").unwrap().as_str()
			.chars()
			.rev()
			.enumerate()
			.map(|(idx,ch)| if ch == 'R' { 1 << idx } else { 0 })
			.sum();
		Ok(Seat
		{
			row,
			column,
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("started with input: {}", self.input);

		let seats = self.input.lines()
			.map(|line| line.parse::<Seat>())
			.filter_map(Result::ok)
			.map(|seat| seat.id())
			.collect::<std::collections::BinaryHeap<_>>()
			.into_sorted_vec();

		for window in seats.windows(2)
		{
			if (window[1] - window[0]) > 1
			{
				return Ok(format!("{}", window[0] + 1));
			}
		}

		bail!(Error::AocNoSolution);
	}
}

