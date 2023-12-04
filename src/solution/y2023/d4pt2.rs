use crate::error::*;

use std::collections::BTreeSet;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d4pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
///     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
///     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
///     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
///     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
///     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "30");
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

#[allow(unused)]
struct Card
{
	id: usize,
	winner: BTreeSet<usize>,
	number: BTreeSet<usize>,
}

impl Card
{
	fn score(&self) -> usize
	{
		self.winner.intersection(&self.number).count()
	}
}

impl std::str::FromStr for Card
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		trace!("parsing card {}", input);
		let input = input.strip_prefix("Card").ok_or(Error::AocParsing)?.trim_start();
		let (id, input) = input.split_once(": ").ok_or(Error::AocParsing)?;
		let (winner, number) = input.split_once(" | ").ok_or(Error::AocParsing)?;
		Ok(Card
		{
			id: id.parse()?,
			winner: winner.split_whitespace().map(str::parse).collect::<std::result::Result<_, _>>()?,
			number: number.split_whitespace().map(str::parse).collect::<std::result::Result<_, _>>()?,
		})
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let cards: Vec<Card> = self.input.lines()
			.map(str::parse)
			.map(|res| res.context(Error::AocParsing))
			.collect::<Result<_>>()?;

		let result: usize = cards.iter()
			.fold((0, Default::default()), |(mut count, mut old): (usize, std::collections::VecDeque<usize>), card|
			{
				debug!("card {}: score={}", card.id, card.score());
				let amount = old.pop_front().unwrap_or(1);
				trace!("card {}: amount={}", card.id, amount);
				count += amount;
				let mut new = std::collections::VecDeque::new();
				for i in 0..card.score()
				{
					let next = old.pop_front().unwrap_or(1);
					trace!("card {}: adding to index {} ({} + {})", card.id, i, next, amount);
					new.push_back(next + amount);
				}
				new.extend(old);
				(count, new)
			}).0;

		Ok(format!("{}", result))
	}
}

