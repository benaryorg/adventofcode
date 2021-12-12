use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d12pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "start-A\n\
///     start-b\n\
///     A-c\n\
///     A-b\n\
///     b-d\n\
///     A-end\n\
///     b-end";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "10");
/// let input = "dc-end\n\
///     HN-start\n\
///     start-kj\n\
///     dc-start\n\
///     dc-HN\n\
///     LN-dc\n\
///     HN-end\n\
///     kj-sa\n\
///     kj-HN\n\
///     kj-dc";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "19");
/// let input = "fs-end\n\
///     he-DX\n\
///     fs-he\n\
///     start-DX\n\
///     pj-DX\n\
///     end-zg\n\
///     zg-sl\n\
///     zg-pj\n\
///     pj-he\n\
///     RW-he\n\
///     fs-DX\n\
///     pj-RW\n\
///     zg-RW\n\
///     start-pj\n\
///     he-WI\n\
///     zg-he\n\
///     pj-fs\n\
///     start-RW";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "226");
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

		let connections: Vec<(&str, &str)> = self.input.lines()
			.map(|line|
			{
				let mut strs = line.split('-');
				Ok(
					( strs.next().ok_or(Error::AocParsing)?
					, strs.next().ok_or(Error::AocParsing)?
					)
				)
			})
			.collect::<Result<Vec<(&str, &str)>>>()?;

		let mut counter = 0;
		let mut open_paths = vec![vec!["start"]];

		while !open_paths.is_empty()
		{
			trace!("open paths: {:?}", open_paths);
			let mut new_paths = Vec::new();
			for path in open_paths
			{
				let current = path.last().ok_or(Error::AocParsing)?;
				new_paths.extend(
					connections.iter()
						.filter_map(|(a, b)|
						{
							if a.eq(current)
							{
								return Some(b);
							}
							if b.eq(current)
							{
								return Some(a);
							}
							None
						})
						.inspect(|next| trace!("{:?} could go {}", path, next))
						.filter(|next|
						{
							if next.chars().next()
								.map(|ch| ch.is_ascii_uppercase())
								.unwrap_or(true)
							{
								return true;
							}
							!path.contains(next)
						})
						.inspect(|next| trace!("{:?} could go {}", path, next))
						.flat_map(|next|
						{
							let mut path = path.clone();
							path.push(next);
							if next.ne(&"end")
							{
								return Some(path);
							}
							debug!("found: {:?}", path);
							counter += 1;
							None
						})
				);
			}
			open_paths = new_paths;
		}

		Ok(format!("{}", counter))
	}
}

