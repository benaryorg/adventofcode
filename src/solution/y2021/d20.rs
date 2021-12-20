use crate::error::*;

use nom::
{
	character::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
	Parser,
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d20::Solution, Solution as S };
/// # env_logger::init();
/// let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
///     \n#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
///     \n.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
///     \n.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
///     \n.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
///     \n...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
///     \n..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\
///     \n\
///     \n#..#.\
///     \n#....\
///     \n##..#\
///     \n..#..\
///     \n..###\
///     \n";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "35");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "3351");
/// ```
pub struct Solution
{
	input: String,
	steps: usize,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			steps: 2,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			steps: 50,
		}
	}
}

type Set<V> = std::collections::HashSet<V>;

fn mask(input: &str) -> IResult<&str, bitvec::vec::BitVec, nom::error::VerboseError<&str>>
{
	fold_many1(terminated(one_of("#."), opt(newline)), bitvec::vec::BitVec::new, |mut acc, ch|
	{
		acc.push(ch == '#');
		acc
	})(input)
}

fn input(input: &str) -> IResult<&str, Set<(isize, isize)>, nom::error::VerboseError<&str>>
{
	let (input, bitlines) = many1(terminated(many1(map(one_of("#."), |ch| ch == '#')), newline))(input)?;

	let set = bitlines.into_iter()
		.enumerate()
		.flat_map(|(y, v)|
		{
			v.into_iter()
				.enumerate()
				.filter_map(move |(x, val)| if !val { None } else { Some((x as isize, y as isize)) })
		})
		.collect();
	Ok((input, set))
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, (mask, input)) = terminated(separated_pair(mask, many1(newline), input), terminated(many0(newline), eof))
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))
			.context(Error::AocParsing)?;

		let invert = *mask.get(0).ok_or(Error::AocNoSolution).context("cannot retrieve invert bit")?;
		let mut odd = true;

		let mut iter = std::iter::successors(Some(input), |input|
		{
			let min_x = input.iter().min_by_key(|(x, _y)| x)?.0 - 1;
			let max_x = input.iter().max_by_key(|(x, _y)| x)?.0 + 1;
			let min_y = input.iter().min_by_key(|(_x, y)| y)?.1 - 1;
			let max_y = input.iter().max_by_key(|(_x, y)| y)?.1 + 1;

			odd = !odd;

			Some((min_x..=max_x)
				.flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
				.filter(|&(x, y)|
				{
					[
						(x-1, y-1),
						(x  , y-1),
						(x+1, y-1),
						(x-1, y  ),
						(x  , y  ),
						(x+1, y  ),
						(x-1, y+1),
						(x  , y+1),
						(x+1, y+1),
					]
						.iter()
						.map(|&(x, y)|
						{
							if x <= min_x || x >= max_x || y <= min_y || y >= max_y
							{
								(invert & odd) as usize
							}
							else
							{
								input.contains(&(x, y)) as usize
							}
						})
						.reduce(|acc, new|
						{
							(acc << 1) | new
						})
						.map(|idx| mask.get(idx))
						.flatten()
						.map(|b| *b)
						.unwrap_or(false)
				})
				.collect())
		});

		let set = iter.nth(self.steps).ok_or(Error::AocNoSolution).context("not enough elements")?;
		Ok(format!("{}", set.len()))
	}
}

