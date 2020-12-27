use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D6Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let input = "abc\n\
///     \n\
///     a\n\
///     b\n\
///     c\n\
///     \n\
///     ab\n\
///     ac\n\
///     \n\
///     a\n\
///     a\n\
///     a\n\
///     a\n\
///     \n\
///     b";
/// assert_eq!(Solution::new(input.to_string()).solve().expect("1"), "6");
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

		let answers: usize = self.input.split("\n\n")
			.map(|group|
			{
				group.lines()
					.map(|person|
					{
						person.chars()
							.collect::<std::collections::BTreeSet<char>>()
					})
					.fold(None,|acc: Option<std::collections::BTreeSet<char>>,elem|
					{
						acc
							.map(|acc|
							{
								acc.intersection(&elem)
									.copied()
									.collect()
							})
							.or(Some(elem))
					})
					.map(|set| set.len())
					.unwrap_or(0)
			})
			.sum();

		Ok(format!("{}", answers))
	}
}

