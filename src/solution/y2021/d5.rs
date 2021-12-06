use crate::error::*;

use nom::
{
	character::complete::*,
	number::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	IResult,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d5::Solution, Solution as S };
/// # env_logger::init();
/// let input = "0,9 -> 5,9\n\
///     8,0 -> 0,8\n\
///     9,4 -> 3,4\n\
///     2,2 -> 2,1\n\
///     7,0 -> 7,4\n\
///     6,4 -> 2,0\n\
///     0,9 -> 2,9\n\
///     3,4 -> 1,4\n\
///     0,0 -> 8,8\n\
///     5,5 -> 8,2";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "5");
/// ```
pub struct Solution
{
	input: String,
	consider_diagonals: bool,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			consider_diagonals: false,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			consider_diagonals: true,
		}
	}
}

fn parse_line(input: &str) -> IResult<&str, ((usize,usize),(usize,usize))>
{
	let (input, (x1, y1)) = separated_pair(double, char(','), double)(input)?;
	let (input, _) = tag(" -> ")(input)?;
	let (input, (x2, y2)) = separated_pair(double, char(','), double)(input)?;
	let (input, _) = eof(input)?;

	Ok((input, ((x1 as usize, y1 as usize), (x2 as usize, y2 as usize))))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let coords = self.input.lines()
			.map(|line| parse_line(line).map_err(|err| anyhow!("{}", err)).map(|coords| coords.1))
			.collect::<Result<Vec<_>>>()?;

		debug!("{:#?}", coords);

		let unfolded = coords.into_iter()
			.filter(|((x1, y1), (x2, y2))| self.consider_diagonals || x1 == x2 || y1 == y2)
			.flat_map(|((x1, y1), (x2, y2))|
			{
				let values_x = ((x1.min(x2))..=(x1.max(x2))).into_iter().collect::<Vec<_>>();
				let values_y = ((y1.min(y2))..=(y1.max(y2))).into_iter().collect::<Vec<_>>();

				if values_x.len() == 1
				{
					return std::iter::repeat(values_x[0]).zip(values_y).collect::<Vec<_>>()
				}
				if values_y.len() == 1
				{
					return values_x.into_iter().zip(std::iter::repeat(values_y[0])).collect::<Vec<_>>()
				}
				values_x.into_iter().zip(values_y).collect::<Vec<_>>()
			})
			.fold(std::collections::BTreeMap::<(usize, usize), usize>::new(), |mut map, tuple|
			{
				*map.entry(tuple).or_default() += 1;
				map
			});

		debug!("{:#?}", unfolded);

		for y in 0..=9
		{
			let line = (0..=9).into_iter()
				.map(|x| unfolded.get(&(x,y)).map(|i| format!("{}", i).chars().last().unwrap()).unwrap_or('.'))
				.collect::<String>();
			info!("{}", line);
		}

		let count = unfolded.into_iter()
			.filter(|(_key, value)| *value >= 2)
			.count();

		Ok(format!("{}", count))
	}
}

