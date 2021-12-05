use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D3Pt2 as Solution,
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
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "336");
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
							_ => bail!(Error::AocParseError),
						})
					})
					.collect::<Result<Vec<_>>>()
			})
			.collect::<Result<Vec<_>>>()?;

		let trees: usize = [(1,1),(3,1),(5,1),(7,1),(1,2)].iter()
			.map(|&(movex,movey)|
			{
				lines.iter()
					.step_by(movey)
					.enumerate()
					.skip(1)
					.map(|(idx,vec)| (idx * movex % vec.len(),vec))
					.filter(|(offset,vec)| vec[*offset])
					.count()
			})
			.product();

		Ok(format!("{}", trees))
	}
}

