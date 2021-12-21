use crate::error::*;

use nom::
{
	character::complete::*,
	number::complete::*,
	bytes::complete::*,
	combinator::*,
	sequence::*,
	multi::*,
	IResult,
	Parser,
};

#[derive(Debug,Eq,PartialEq)]
enum Part
{
	Part1,
	Part2,
}

// TODO: day 19 is missing so far

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2021::d21::Solution, Solution as S };
/// # env_logger::init();
/// let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
/// assert_eq!(Solution::part1(input.to_string()).solve().unwrap(), "739785");
/// assert_eq!(Solution::part2(input.to_string()).solve().unwrap(), "444356092776315");
/// ```
pub struct Solution
{
	input: String,
	part: Part,
}

impl Solution
{
	pub fn part1(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part1,
		}
	}

	pub fn part2(input: String) -> Self
	{
		Self
		{
			input,
			part: Part::Part2,
		}
	}
}

fn position(input: &str) -> IResult<&str, usize>
{
	preceded(tuple((tag("Player "), double, tag(" starting position: "))), map(double, |d| d as usize))(input)
}

trait Die
{
	fn roll(&mut self) -> usize;
	fn count(&self) -> usize;
}

#[derive(Debug, Default)]
struct DeterministicDie
{
	step: usize,
}

impl Die for DeterministicDie
{
	fn roll(&mut self) -> usize
	{
		self.step += 1;
		(self.step - 1) % 100 + 1
	}

	fn count(&self) -> usize
	{
		self.step
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let (_, positions) = all_consuming(terminated(separated_list1(newline, position), opt(newline)))
			.parse(&self.input)
			.map_err(|err| anyhow!("{}", err))?;

		let mut players = positions.into_iter().map(|pos| (0, pos)).collect::<Vec<_>>();

		if self.part == Part::Part1
		{
			let mut die = DeterministicDie::default();
			'outer: loop
			{
				for (ref mut score, ref mut pos) in players.iter_mut()
				{
					let roll = die.roll() + die.roll() + die.roll();
					*pos = (*pos + roll - 1) % 10 + 1;
					*score += *pos;
					if *score >= 1000
					{
						break 'outer;
					}
				}
			}

			let result = players.into_iter().map(|(score, _)| score).min().ok_or(Error::AocNoSolution)? * die.count();
			Ok(format!("{}", result))
		}
		else
		{
			assert_eq!(players.len(), 2);
			let mut player_wins = vec![0_u128; players.len()];
			let mut games = std::collections::HashMap::<(bool, (usize, usize), (usize, usize)), u128>::new();
			games.insert((true, players[0], players[1]), 1);

			while let Some((game, count)) = games.remove_entry(&games.keys().min_by_key(|(_, (s1, _), (s2, _))| s1 + s2).copied().unwrap_or_default())
			{
				let player1 = game.0;
				let (score, pos) = if player1 { game.1 } else { game.2 };
				for roll in (1..=3)
					.flat_map(move |first| (1..=3).flat_map(move |second| (1..=3).map(move |third| (first + second + third))))
				{
					let pos = (pos + roll - 1) % 10 + 1;
					let score = score + pos;
					if score >= 21
					{
						player_wins[(!player1) as usize] += count;
						trace!("{} wins for player {}", count, (!player1) as usize);
						continue;
					}
					let key = if player1
					{
						(false, (score, pos), game.2)
					}
					else
					{
						(true, game.1, (score, pos))
					};
					*games.entry(key).or_insert(0) += count;
				}
			}

			Ok(format!("{}", player_wins.into_iter().max().unwrap()))
		}
	}
}

