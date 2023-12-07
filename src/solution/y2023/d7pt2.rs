use crate::error::*;

use ::
{
	std::
	{
		convert::TryFrom,
		cmp::Ordering,
	},
};

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d7pt2::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     32T3K 765\n\
///     T55J5 684\n\
///     KK677 28\n\
///     KTJJT 220\n\
///     QQQJA 483";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "5905");
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum Card
{
	A,
	K,
	Q,
	J,
	T,
	Number(u32),
}

impl From<Card> for u32
{
	fn from(card: Card) -> u32
	{
		match card
		{
			Card::A => 13,
			Card::K => 12,
			Card::Q => 11,
			Card::J => 1,
			Card::T => 10,
			Card::Number(num) => num,
		}
	}
}

impl PartialOrd for Card
{
    fn partial_cmp(&self, o: &Self) -> Option<Ordering>
    {
		Some(self.cmp(o))
    }
}

impl Ord for Card
{
    fn cmp(&self, other: &Self) -> Ordering
    {
		let s: u32 = self.clone().into();
		let o: u32 = other.clone().into();
		s.cmp(&o)
    }
}

impl TryFrom<char> for Card
{
	type Error = Error;
	fn try_from(input: char) -> std::result::Result<Self, Error>
	{
		match input
		{
			'A' => Ok(Card::A),
			'K' => Ok(Card::K),
			'Q' => Ok(Card::Q),
			'J' => Ok(Card::J),
			'T' => Ok(Card::T),
			'2'..='9' => Ok(Card::Number(input.to_digit(10).unwrap())),
			_ => Err(Error::AocParsing),
		}
	}
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Set
{
	Five(Vec<Card>),
	Four(Vec<Card>),
	Full(Vec<Card>),
	Three(Vec<Card>),
	TwoPair(Vec<Card>),
	Two(Vec<Card>),
	Nothing(Vec<Card>),
	/*
	 * in case you ever want to make a proper hand comparison that isn't scuffed af,
	 * use this here and replace the construction of the Set in from_str() with the content in trace!
	 */
	// Five(Card),
	// Four((Card, Card)),
	// Full((Card, Card)),
	// Three((Card, Card, Card)),
	// TwoPair((Card, Card, Card)),
	// Two((Card, Card, Card, Card)),
	// Nothing(Vec<Card>),
}

impl std::str::FromStr for Set
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		trace!("card parser input: {:?}", input);
		if input.len() != 5 { Err(Error::AocParsing)? };
		let cards = input.chars().map(|ch| Ok(Card::try_from(ch)?)).collect::<Result<Vec<Card>>>()?;
		trace!("parsing cards: {:?}", cards);

		let map = cards.iter()
			.cloned()
			.fold(std::collections::BTreeMap::<Card, usize>::new(), |mut map, card|
			{
				*map.entry(card).or_default() += 1;
				map
			});

		if let Some((card, _)) = map.iter().rev().find(|&(_, count)| *count == 5)
		{
			trace!("found five: {:?}", card);
			return Ok(Set::Five(cards));
		}
		if let (Some((four, _)), Some((one, _))) = (map.iter().rev().find(|&(_, count)| *count == 4), map.iter().rev().find(|&(_, count)| *count == 1))
		{
			trace!("found four: {:?} & {:?}", four, one);
			return Ok(Set::Four(cards));
		}
		if let (Some((three, _)), Some((two, _))) = (map.iter().rev().find(|&(_, count)| *count == 3), map.iter().rev().find(|&(_, count)| *count == 2))
		{
			trace!("found full house: {:?} & {:?}", three, two);
			return Ok(Set::Full(cards));
		}
		if let Some((three, _)) = map.iter().rev().find(|&(_, count)| *count == 3)
		{
			let mut rest = map.iter().rev().filter(|&(_, count)| *count == 1);
			let o1 = rest.next().unwrap().0;
			let o2 = rest.next().unwrap().0;
			trace!("found three: {:?} & {:?} & {:?}", three, o1, o2);
			return Ok(Set::Three(cards));
		}
		if map.iter().rev().filter(|&(_, count)| *count == 2).count() == 2
		{
			let mut twos = map.iter().rev().filter(|&(_, count)| *count == 2);
			let t1 = twos.next().unwrap().0;
			let t2 = twos.next().unwrap().0;
			let one = map.iter().rev().find(|&(_, count)| *count == 1).unwrap().0;
			trace!("found two pairs: {:?} & {:?} & {:?}", t1, t2, one);
			return Ok(Set::TwoPair(cards));
		}
		if let Some((two, _)) = map.iter().rev().find(|&(_, count)| *count == 2)
		{
			let mut rest = map.iter().rev().filter(|&(_, count)| *count == 1);
			let o1 = rest.next().unwrap().0;
			let o2 = rest.next().unwrap().0;
			let o3 = rest.next().unwrap().0;
			trace!("found pair: {:?} & {:?} & {:?} & {:?}", two, o1, o2, o3);
			return Ok(Set::Two(cards));
		}
		trace!("found pair: {:?}", cards);
		Ok(Set::Nothing(cards))
	}
}

impl PartialOrd for Set
{
    fn partial_cmp(&self, o: &Self) -> Option<Ordering>
    {
		Some(self.cmp(o))
    }
}

impl Ord for Set
{
    fn cmp(&self, other: &Self) -> Ordering
    {
		match (self, other)
		{
			(Set::Five(l), Set::Five(r)) => l.cmp(r),
			(Set::Five(_), _) => Ordering::Greater,
			(_, Set::Five(_)) => Ordering::Less,
			(Set::Four(l), Set::Four(r)) => l.cmp(r),
			(Set::Four(_), _) => Ordering::Greater,
			(_, Set::Four(_)) => Ordering::Less,
			(Set::Full(l), Set::Full(r)) => l.cmp(r),
			(Set::Full(_), _) => Ordering::Greater,
			(_, Set::Full(_)) => Ordering::Less,
			(Set::Three(l), Set::Three(r)) => l.cmp(r),
			(Set::Three(_), _) => Ordering::Greater,
			(_, Set::Three(_)) => Ordering::Less,
			(Set::TwoPair(l), Set::TwoPair(r)) => l.cmp(r),
			(Set::TwoPair(_), _) => Ordering::Greater,
			(_, Set::TwoPair(_)) => Ordering::Less,
			(Set::Two(l), Set::Two(r)) => l.cmp(r),
			(Set::Two(_), _) => Ordering::Greater,
			(_, Set::Two(_)) => Ordering::Less,
			(Set::Nothing(l), Set::Nothing(r)) => l.cmp(r),
		}
    }
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {}", self.input);

		let players = self.input.lines()
			.map(|line|
			{
				let (cards, score) = line.split_once(' ').ok_or(Error::AocParsing)?;
				let original_cards = cards.chars().map(|ch| Ok(Card::try_from(ch)?)).collect::<Result<Vec<Card>>>()?;
				let cards = "23456789TJQKA".chars()
					.map(String::from)
					.map(|r| cards.replace('J', &r))
					.map(|cards| cards.parse::<Set>().context(Error::AocParsing))
					.collect::<Result<Vec<Set>>>()?;
				let cards = match cards.into_iter().max().unwrap()
				{
					Set::Five(_) => Set::Five(original_cards),
					Set::Four(_) => Set::Four(original_cards),
					Set::Full(_) => Set::Full(original_cards),
					Set::Three(_) => Set::Three(original_cards),
					Set::TwoPair(_) => Set::TwoPair(original_cards),
					Set::Two(_) => Set::Two(original_cards),
					Set::Nothing(_) => Set::Nothing(original_cards),
				};
				let score = score.parse::<usize>().context(Error::AocParsing)?;
				Ok((cards, score))
			})
			.collect::<Result<std::collections::BTreeSet<(Set, usize)>>>()?;

		let result: usize = players.into_iter().enumerate()
			.inspect(|(idx, (set, score))| trace!("rank {}: {:?} (score: {})", idx + 1, set, score))
			.map(|(idx, (_, score))| score * (idx + 1))
			.sum();

		Ok(format!("{}", result))
	}
}

