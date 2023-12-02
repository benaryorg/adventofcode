use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d2pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
///     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
///     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
///     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
///     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "8");
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

		let games = self.input.lines()
			.map(|line|
			{
				trace!("parsing line: {:?}", line);
				let line = line.strip_prefix("Game ").ok_or(Error::AocParsing)?;
				let (game, line) = line.split_once(": ").ok_or(Error::AocParsing)?;
				let game: usize = game.parse().context(Error::AocParsing)?;
				let sets = line.split("; ")
					.map(|game|
					{
						game.split(", ")
							.map(|draw|
							{
								let (num, colour) = draw.split_once(' ').ok_or(Error::AocParsing)?;
								let num = num.parse::<usize>().context(Error::AocParsing)?;
								Ok((colour.to_string(), num))
							})
							.collect::<Result<std::collections::HashMap<String, usize>>>()
					})
					.collect::<Result<Vec<_>>>()?;
				Ok((game, sets))
			})
			.collect::<Result<std::collections::BTreeMap<usize, _>>>()?;

		let bag: std::collections::HashMap<String, usize> = [("red", 12), ("green", 13), ("blue", 14)].iter()
			.map(|(s, n)| (s.to_string(), *n))
			.collect();

		let result: usize = games.iter()
			.filter(|(id, sets)|
			{
				let valid = sets.iter()
					.flatten()
					.all(|(colour, number)| bag.get(colour).copied().unwrap_or_default() >= *number);
				trace!("game {} is {} ({:?})", id, if valid { "valid" } else { "invalid" }, sets);
				valid
			})
			.map(|(id, _)| id)
			.sum();

		Ok(format!("{}", result))
	}
}

