use crate::error::*;

/// # Examples
///
/// Does not loop indefinitely:
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D22Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// let example = "Player 1:\n\
///     43\n\
///     19\n\
///     \n\
///     Player 2:\n\
///     2\n\
///     29\n\
///     14";
/// assert!(Solution::new(example.to_string()).solve().is_ok());
/// ```
///
/// Does yield correct result:
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D22Pt2 as Solution,
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
/// assert_eq!(Solution::new(example.to_string()).solve().expect("1"), "291");
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

#[derive(Clone,Debug)]
enum Player
{
	One,
	Two,
}

type Deck = std::collections::VecDeque<usize>;

fn recursive_combat(mut player1: Deck, mut player2: Deck) -> (Player,Deck)
{
	let mut states = std::collections::HashSet::new();

	debug!("=== Game N ===");

	while !player1.is_empty() && !player2.is_empty()
	{
		debug!("");
		debug!("-- Round N (Game N) --");
		debug!("Player 1's deck: {}", player1.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
		debug!("Player 2's deck: {}", player2.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
		if states.contains(&(player1.clone(),player2.clone()))
		{
			return (Player::One,player1);
		}

		states.insert((player1.clone(),player2.clone()));

		let card1 = player1.pop_front().unwrap();
		let card2 = player2.pop_front().unwrap();
		debug!("Player 1 plays: {}", card1);
		debug!("Player 2 plays: {}", card2);

		let winner = if player1.len() >= card1 && player2.len() >= card2
		{
			debug!("recursion started!");
			recursive_combat(player1.iter().take(card1).copied().collect(), player2.iter().take(card2).copied().collect()).0
		}
		else
		{
			if card1 > card2
			{
				Player::One
			}
			else
			{
				Player::Two
			}
		};
		debug!("winner is {:?}", winner);

		match winner
		{
			Player::One => player1.extend(&[card1,card2]),
			Player::Two => player2.extend(&[card2,card1]),
		};
	}

	if player1.is_empty()
	{
		(Player::Two,player2)
	}
	else
	{
		(Player::One,player1)
	}	
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let mut parts = self.input.split("\n\n");

		let player1 = parts.next().ok_or(Error::AocParseError)?.lines()
			.skip(1)
			.map(|s| Ok(s.parse::<usize>()?))
			.collect::<Result<Deck>>()?;
		let player2 = parts.next().ok_or(Error::AocParseError)?.lines()
			.skip(1)
			.map(|s| Ok(s.parse::<usize>()?))
			.collect::<Result<Deck>>()?;

		let winning_deck = recursive_combat(player1,player2).1;

		Ok(format!("{}", winning_deck.iter().rev().zip(1..).map(|(i1,i2)| i1*i2).sum::<usize>()))
	}
}

