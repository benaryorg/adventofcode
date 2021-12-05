use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D22Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let example = "Player 1:\n\
///     9\n\
///     2\n\
///     6\n\
///     3\n\
///     1\n\
///     \n\
///     Player 2:\n\
///     5\n\
///     8\n\
///     4\n\
///     7\n\
///     10";
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "306");
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

		let mut parts = self.input.split("\n\n");

		let mut player1 = parts.next().ok_or(Error::AocParsing)?.lines()
			.skip(1)
			.map(|s| Ok(s.parse::<usize>()?))
			.collect::<Result<std::collections::VecDeque<_>>>()?;
		let mut player2 = parts.next().ok_or(Error::AocParsing)?.lines()
			.skip(1)
			.map(|s| Ok(s.parse::<usize>()?))
			.collect::<Result<std::collections::VecDeque<_>>>()?;

		while !player1.is_empty() && !player2.is_empty()
		{
			let card1 = player1.pop_front().unwrap();
			let card2 = player2.pop_front().unwrap();

			let winner = if card1 > card2
			{
				&mut player1
			}
			else
			{
				&mut player2
			};

			(*winner).push_back(card1.max(card2));
			(*winner).push_back(card1.min(card2));
		}

		Ok(format!("{}", player1.iter().chain(player2.iter()).zip((1..=player1.len().max(player2.len())).rev()).map(|(i1,i2)| i1*i2).sum::<usize>()))
	}
}

