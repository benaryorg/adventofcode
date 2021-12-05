use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D7Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
///     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
///     bright white bags contain 1 shiny gold bag.\n\
///     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
///     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
///     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
///     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
///     faded blue bags contain no other bags.\n\
///     dotted black bags contain no other bags.";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "32");
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

		let map = self.input.lines()
			.map(|line|
			{
				let idx = line.find(" bags ").ok_or(Error::AocParseError)?;
				let colour = line.get(..idx).ok_or(Error::AocParseError)?;
				let rest = line.get((idx+14)..).ok_or(Error::AocParseError)?;
				let content = rest.strip_suffix('.').ok_or(Error::AocParseError)?.split(", ")
					.filter(|&s| s != "no other bags")
					.map(|bag|
					{
						let content = bag.strip_suffix(" bag").or_else(|| bag.strip_suffix(" bags")).ok_or(Error::AocParseError)?;
						let count: usize = content.split(' ').next().ok_or(Error::AocParseError)?.parse()?;
						let colour = content.splitn(2,' ').skip(1).next().ok_or(Error::AocParseError)?;
						Ok((count,colour))
					})
					.collect::<Result<Vec<_>>>()?;
				Ok((colour,content))
			})
			.collect::<Result<std::collections::BTreeMap<_,_>>>()?;

		let mut total = 0;
		let mut vec = vec![(1,"shiny gold")];
		while let Some((count,colour)) = vec.pop()
		{
			total += count;
			if let Some(bags) = map.get(colour)
			{
				vec.extend(bags.iter().map(|&(cnt,colour)| (cnt * count, colour)));
			}
		}

		Ok(format!("{}", total-1))
	}
}

