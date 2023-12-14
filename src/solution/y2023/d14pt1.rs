use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d14pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input =
///     [ "O....#...."
///     , "O.OO#....#"
///     , ".....##..."
///     , "OO.#O....O"
///     , ".O.....O#."
///     , "O.#..O.#.#"
///     , "..O..#O..O"
///     , ".......O.."
///     , "#....###.."
///     , "#OO..#...."
///     ].join("\n");
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "136");
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

		let cols = self.input.lines()
			.fold(Vec::<Vec<u8>>::new(), |mut vec, line|
			{
				line.bytes()
					.enumerate()
					.for_each(|(idx, b)|
					{
						if vec.len() < idx + 1
						{
							vec.push(Vec::new());
						}
						vec.get_mut(idx).unwrap().push(b);
					});
				vec
			});

		let result: usize = cols.iter()
			.enumerate()
			.map(|(col_idx, col)|
			{
				debug!("calculating col {}: {:?}", col_idx, String::from_utf8_lossy(col));
				col.iter()
					.copied()
					.enumerate()
					.fold((0, col.len()), |(load, weight), (idx, b)|
					{
						match b
						{
							b'#' =>
							{
								let weight = col.len() - idx - 1;
								trace!("{}: found static, setting weight to {}", col_idx, weight);
								(load, weight)
							},
							b'O' =>
							{
								trace!("{}: found round, increasing load by {}", col_idx, weight);
								(load + weight, weight - 1)
							},
							b'.' =>
							{
								(load, weight)
							},
							_ => unreachable!(),
						}
					}).0
			})
			.sum();

		Ok(format!("{}", result))
	}
}

