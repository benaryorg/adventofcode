use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d3pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     467..114..\n\
///     ...*......\n\
///     ..35..633.\n\
///     ......#...\n\
///     617*......\n\
///     .....+.58.\n\
///     ..592.....\n\
///     ......755.\n\
///     ...$.*....\n\
///     .664.598..";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "4361");
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

struct Cell
{
	content: CellContent,
	neighbours: Vec<CellContent>,
}

#[derive(Debug)]
enum CellContent
{
	Number(usize),
	Symbol(char),
}

impl CellContent
{
	fn is_symbol(&self) -> bool
	{
		match self
		{
			CellContent::Symbol(_) => true,
			_ => false,
		}
	}
}

fn neighbours(len: isize) -> Vec<(isize, isize)>
{
	// left speck
	std::iter::once((-1, 0))
		// right speck
		.chain(std::iter::once((len, 0)))
		// top row
		.chain((-1..=len).map(|x| (x, -1)))
		// bottom row
		.chain((-1..=len).map(|x| (x, 1)))
		.collect()
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let char_grid: Vec<Vec<char>> = self.input.lines().map(str::chars).map(Iterator::collect::<Vec<_>>).collect();
		let char_grid = &char_grid;

		let cells: Vec<Cell> = char_grid.iter()
			.enumerate()
			.flat_map(|(y, row): (usize, _)|
			{
				row.iter()
					.enumerate()
					.filter_map(move |(x, ch)|
					{
						let row = char_grid.get(y).unwrap();
						match ch
						{
							'.' =>
							{
								trace!("nothing at ({}, {})", x, y);
								None
							},
							'0'..='9' if !row.get(((x as isize) - 1) as usize).map(char::is_ascii_digit).unwrap_or(false) =>
							{
								let length = row.iter()
									.skip(x)
									.copied()
									.take_while(char::is_ascii_digit)
									.count();

								let num = row[x..(x + length)].iter()
									.map(|ch| ch.to_digit(10).unwrap() as usize)
									.reduce(|a, b| a * 10 + b)
									.unwrap();

								debug!("number: {} (len: {}) at ({}, {})", num, length, x, y);

								let neighbours = neighbours(length as isize)
									.iter()
									.map(|(dx, dy)| (((x as isize)+dx) as usize, ((y as isize)+dy) as usize))
									.filter_map(|(x, y)|
									{
										char_grid
											.get(y)
											.and_then(|row| row.get(x))
											.and_then(|ch|
											{
												match ch
												{
													'.' => None,
													'0'..='9' => Some(CellContent::Number(0)),
													&ch => Some(CellContent::Symbol(ch)),
												}
											})
									})
									.collect();

								debug!("neighbours for {}: {:?}", num, neighbours);

								Some(Cell
								{
									content: CellContent::Number(num),
									neighbours,
								})
							},
							'0'..='9' =>
							{
								trace!("continued number: {:?} at ({}, {})", ch, x, y);
								None
							},
							&ch =>
							{
								trace!("symbol: {:?} at ({}, {})", ch, x, y);
								let neighbours = neighbours(1)
									.iter()
									.map(|(dx, dy)| (((x as isize)+dx) as usize, ((y as isize)+dy) as usize))
									.filter_map(|(x, y)|
									{
										char_grid
											.get(y)
											.and_then(|row| row.get(x))
											.and_then(|ch|
											{
												match ch
												{
													'.' => None,
													'0'..='9' => Some(CellContent::Number(0)),
													&ch => Some(CellContent::Symbol(ch)),
												}
											})
									})
									.collect();
								Some(Cell
								{
									content: CellContent::Symbol(ch),
									neighbours,
								})
							},
						}
					})
			})
			.collect();

		let result: usize = cells.iter()
			.filter_map(|cell|
			{
				if let CellContent::Number(num) = cell.content
				{
					cell.neighbours.iter().any(CellContent::is_symbol).then_some(num)
				}
				else
				{
					None
				}
			})
			.sum();

		Ok(format!("{}", result))
	}
}

