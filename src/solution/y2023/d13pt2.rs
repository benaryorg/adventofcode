use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d13pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\
///     \n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "400");
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
						let count = top.iter()
							.rev()
							.zip(bottom.iter())
							.inspect(|(top, bottom)| trace!("comparing {:?} and {:?}", top, bottom))
							.map(|(top, bottom)|
							{
								top.chars()
									.zip(bottom.chars())
									.filter(|(a, b)| a != b)
									.count()
							})
							.sum::<usize>();
						(count == 1).then_some(i * 100)
					})
					.or_else(||
					{
						(1..block[0].len())
							.find(|&i|
							{
								let count = block.iter()
									.map(|line|
									{
										let (left, right) = line.split_at(i);
										trace!("comparing {:?} and {:?}", left, right);
										left.chars()
											.rev()
											.zip(right.chars())
											.inspect(|(top, bottom)| trace!("comparing {:?} and {:?}", top, bottom))
											.filter(|(left, right)| left != right)
											.count()
									})
									.sum::<usize>();
								count == 1
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

