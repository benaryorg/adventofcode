use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d13pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "6,10\n\
///     0,14\n\
///     9,10\n\
///     0,3\n\
///     10,4\n\
///     4,11\n\
///     6,0\n\
///     6,12\n\
///     4,1\n\
///     0,13\n\
///     10,12\n\
///     3,4\n\
///     3,0\n\
///     8,4\n\
///     1,10\n\
///     2,14\n\
///     8,10\n\
///     9,0\n\
///     \n\
///     fold along y=7\n\
///     fold along x=5";
/// let output = "\
/// #   #####|\n\
/// #   #   #|\n\
/// #   #   #|\n\
/// #   #   #|\n\
/// #   #####|\n\
/// ";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), output);
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

enum Fold
{
	X(usize),
	Y(usize),
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut parts = self.input.splitn(2, "\n\n");
		let mut coords = parts.next().ok_or(Error::AocParsing).context("cannot find coords")?.lines()
			.map(|s|
			{
				let mut split = s.splitn(2, ',');
				let x = split.next()
					.ok_or(Error::AocParsing)
					.context("cannot find x in coords")?
					.parse()?;
				let y = split.next()
					.ok_or(Error::AocParsing)
					.context("cannot find y in coords")?
					.parse()?;
				Ok((x, y))
			})
			.collect::<Result<std::collections::BTreeSet<(usize, usize)>>>()?;

		let folds = parts.next().ok_or(Error::AocParsing)?.lines()
			.map(|s|
			{
				let mut parts = s.strip_prefix("fold along ")
					.ok_or(Error::AocParsing)
					.context("cannot find fold prefix")?
					.split('=');
				let axis = parts.next()
					.ok_or(Error::AocParsing)
					.context("axis cannot be found in fold")?;
				let pos = parts.next()
					.ok_or(Error::AocParsing)
					.context("position not found in fold")?
					.parse()
					.context("position in fold not a number")?;

				match axis
				{
					"x" => Ok(Fold::X(pos)),
					"y" => Ok(Fold::Y(pos)),
					_ => bail!(Error::AocParsing),
				}
			})
			.collect::<Result<Vec<_>>>()?;

		for fold in folds.into_iter()
		{
			coords = coords.into_iter()
				.map(|(x, y)|
				{
					match fold
					{
						Fold::X(fold) => if x > fold { return (fold-(x-fold), y); },
						Fold::Y(fold) => if y > fold { return (x, fold-(y-fold)); },
					}
					(x, y)
				})
				.collect();
		}

		let x_min = *coords.iter().map(|(x, _)| x).min().ok_or(Error::AocParsing)?;
		let x_max = *coords.iter().map(|(x, _)| x).max().ok_or(Error::AocParsing)?;
		let y_min = *coords.iter().map(|(_, y)| y).min().ok_or(Error::AocParsing)?;
		let y_max = *coords.iter().map(|(_, y)| y).max().ok_or(Error::AocParsing)?;
		let coords = &coords;

		let output = (y_min..=y_max)
			.flat_map(move |y|
			{
				(x_min..=x_max)
					.map(move |x|
					{
						if coords.contains(&(x, y))
						{
							'#'
						}
						else
						{
							' '
						}
					})
					.chain("|\n".chars())
			})
			.collect::<String>();

		Ok(output)
	}
}

