use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d12pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "start-A\n\
///     start-b\n\
///     A-c\n\
///     A-b\n\
///     b-d\n\
///     A-end\n\
///     b-end";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "36");
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "103");
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "3509");
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

		let translation = connections.iter()
			.flat_map(|&(a, b)| [a, b])
			.collect::<std::collections::HashSet<_>>()
			.into_iter()
			.enumerate()
			.map(|(int, string)| (string, (int, string.chars().next().unwrap().is_ascii_uppercase())))
			.collect::<std::collections::HashMap<_,_>>();

		trace!("translation: {:?}", translation);

		let start = translation.get("start").copied().unwrap();
		let end = translation.get("end").copied().unwrap();

		let connections = connections.into_iter()
			.map(|(a, b)|
			{
				let a = translation.get(a).unwrap();
				let b = translation.get(b).unwrap();
				(a, b)
			})
			.collect::<Vec<_>>();

		let mut counter = 0;
		let mut paths = vec![(false, vec![start])];

		while !paths.is_empty()
		{
			let mut new_paths = Vec::new();

			paths.into_iter()
				.flat_map(|elem|
				{
					std::iter::repeat(elem)
						.zip(connections.iter().copied())
				})
				.for_each(|((repeat, path), (a, b))|
				{
					if path.len() > 1 && (start.eq(a) || start.eq(b))
					{
						return;
					}
					let current = path.last().copied().unwrap();
					let next = if current.eq(a)
					{
						b
					}
					else
					{
						if current.eq(b)
						{
							a
						}
						else
						{
							return;
						}
					};
					if end.eq(next)
					{
						counter += 1;
						return;
					}
					let repeat = if !next.1
					{
						let max_repeats = if repeat { 1 } else { 2 };
						let current_repeats = path.iter().filter(|elem| next.eq(elem)).take(max_repeats).count();
						if current_repeats >= max_repeats
						{
							return;
						}
						repeat || current_repeats >= 1
					}
					else
					{
						repeat
					};
					let mut path = path.clone();
					path.push(*next);
					new_paths.push((repeat, path))
				});

			paths = new_paths;
		}

		Ok(format!("{}", counter))
	}
}

