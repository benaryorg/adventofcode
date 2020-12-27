use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D3Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "..##.......\n\
/// #   #...#...#..\n\
/// #   .#....#..#.\n\
/// #   ..#.#...#.#\n\
/// #   .#...##..#.\n\
/// #   ..#.##.....\n\
/// #   .#.#.#....#\n\
/// #   .#........#\n\
/// #   #.##...#...\n\
/// #   #...##....#\n\
///     .#..#...#.#";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "7");
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
		debug!("started with input: {}", self.input);

		let lines = self.input.lines()
			.map(|line|
			{
				line.chars()
					.map(|ch|
					{
						Ok(match ch
						{
							'.' => false,
							'#' => true,
							_ => bail!(ErrorKind::ParseError),
						})
					})
					.collect::<Result<Vec<_>>>()
			})
			.collect::<Result<Vec<_>>>()?;

		let trees = lines.iter()
			.enumerate()
			.skip(1)
			.map(|(idx,vec)| (idx * 3 % vec.len(),vec))
			.filter(|(offset,vec)| vec[*offset])
			.count();

		Ok(format!("{}", trees))
	}
}

