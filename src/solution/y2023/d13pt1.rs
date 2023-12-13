use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d13pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\
///     \n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "405");
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

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let inputs = self.input.split("\n\n")
			.map(|block|
			{
				block.lines()
					.map(|line| line.to_string())
					.collect::<Vec<_>>()
			})
			.collect::<Vec<Vec<String>>>();

		let result: usize = inputs.iter()
			.map(|block|
			{
				(1..block.len())
					.find_map(|i|
					{
						let (top, bottom) = block.split_at(i);
						top.iter()
							.rev()
							.zip(bottom.iter())
							.inspect(|(top, bottom)| trace!("comparing {:?} and {:?}", top, bottom))
							.all(|(top, bottom)| top.eq(bottom))
							.then_some(i * 100)
					})
					.or_else(||
					{
						(1..block[0].len())
							.find(|&i|
							{
								block.iter()
									.all(|line|
									{
										let (left, right) = line.split_at(i);
										trace!("comparing {:?} and {:?}", left, right);
										left.chars()
											.rev()
											.zip(right.chars())
											.inspect(|(top, bottom)| trace!("comparing {:?} and {:?}", top, bottom))
											.all(|(left, right)| left.eq(&right))
									})
							})
					})
					.unwrap()
			})
			.enumerate()
			.map(|(id, result)|
			{
				debug!("block {}: {}", id, result);
				result
			})
			.sum();

		Ok(format!("{}", result))
	}
}

