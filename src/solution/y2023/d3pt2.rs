use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d3pt2::Solution, Solution as S };
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "467835");
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
	location: Vec<(usize, usize)>,
	neighbours: Vec<CellContent>,
}

#[derive(Debug, Clone)]
enum CellContent
{
	Number(usize),
	Symbol(char),
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

		let numbers: Vec<Cell> = char_grid.iter()
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
									location: (x..(x+length)).map(|x| (x, y)).collect(),
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
								None
							},
						}
					})
			})
			.collect();
		let numbers = &numbers;

		let symbols: Vec<Cell> = char_grid.iter()
			.enumerate()
			.flat_map(|(y, row): (usize, _)|
			{
				row.iter()
					.enumerate()
					.filter_map(move |(x, ch)|
					{
						match ch
						{
							'.' => None,
							'0'..='9' => None,
							&ch =>
							{
								debug!("symbol: {:?} at ({}, {})", ch, x, y);

								let neighbours = neighbours(1)
									.iter()
									.map(|(dx, dy)| (((x as isize)+dx) as usize, ((y as isize)+dy) as usize))
									.collect::<Vec<_>>();
								let neighbour_numbers = numbers.iter()
									.filter(|cell| cell.location.iter().any(|cl| neighbours.iter().any(|n| n == cl)))
									.map(|cell| cell.content.clone())
									.collect::<Vec<_>>();
								debug!("num numbers around {:?}: {}", ch, neighbour_numbers.len());
								Some(Cell
								{
									content: CellContent::Symbol(ch),
									location: vec![(x, y)],
									neighbours: neighbour_numbers,
								})
							},
						}
					})
			})
			.collect();

		let result: usize = symbols.iter()
			.filter_map(|cell|
			{
				if let CellContent::Symbol(_) = cell.content
				{
					let numbers = cell.neighbours.iter()
						.filter_map(|x|
						{
							if let CellContent::Number(num) = x
							{
								Some(*num)
							}
							else
							{
								None
							}
						})
						.collect::<Vec<_>>();
					(numbers.len() == 2).then_some(numbers.iter().product::<usize>())
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

