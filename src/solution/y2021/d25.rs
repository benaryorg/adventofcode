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

#[derive(Debug,Eq,PartialEq)]
enum Part
{
	Part1,
	Part2,
}

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d25::Solution, Solution as S };
/// # env_logger::init();
/// let input = "v...>>.vv>\
///     \n.vv>>.vv..\
///     \n>>.>v>...v\
///     \n>>v>>.>.v.\
///     \nv>v.vv.v..\
///     \n>.>>..v...\
///     \n.vv..>.>v.\
///     \nv.v..>>v.v\
///     \n....v..v.>\n";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "58");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part1,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part2,
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction
{
	East,
	South,
}

fn direction(input: &str) -> IResult<&str, Option<Direction>>
{
	map(one_of(".>v"), |ch|
	{
		match ch
		{
			'.' => None,
			'v' => Some(Direction::South),
			'>' => Some(Direction::East),
			_ => unreachable!(),
		}
	})(input)
}

fn input(input: &str) -> IResult<&str, Vec<Vec<Option<Direction>>>>
{
	many1(terminated(many1(direction), newline))(input)
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, field) = all_consuming(input)
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		if self.part == Part::Part1
		{
			let count = std::iter::successors(Some(field), |oldfield|
			{
				let mut field = oldfield.clone();
				let mut changed = false;
				for pass in [Direction::East, Direction::South]
				{
					let oldfield = field.clone();
					for y in 0..field.len()
					{
						for x in 0..field[y].len()
						{
							let ylen = field.len();
							let xlen = field[y].len();

							match oldfield[y][x]
							{
								Some(Direction::East) if pass == Direction::East =>
								{
									if oldfield[y][(x + 1) % xlen].is_none()
									{
										trace!("moving {}/{} to {}/{}", x, y, (x + 1) % xlen, y);
										let value = field[y][x].take();
										field[y][(x + 1) % xlen] = value;
										changed = true;
									}
									else
									{
										field[y][x] = oldfield[y][x];
									}
								},
								Some(Direction::South) if pass == Direction::South =>
								{
									if oldfield[(y + 1) % ylen][x].is_none()
									{
										trace!("moving {}/{} to {}/{}", x, y, x, (y + 1) % ylen);
										let value = field[y][x].take();
										field[(y + 1) % ylen][x] = value;
										changed = true;
									}
									else
									{
										field[y][x] = oldfield[y][x];
									}
								},
								_ => {},
							}
						}
					}
				}
				if changed
				{
					Some(field)
				}
				else
				{
					None
				}
			})
				.inspect(|field|
				{
					let s = field.iter()
						.flat_map(|row|
						{
							std::iter::once('\n')
								.chain(
									row.iter()
										.map(|o|
										{
											match o
											{
												None => '.',
												Some(Direction::East) => '>',
												Some(Direction::South) => 'v',
											}
										})
								)
						})
						.collect::<String>();
					debug!("{}", s);
				})
				.count();

			Ok(format!("{}", count))
		}
		else
		{
			unimplemented!()
		}
	}
}

