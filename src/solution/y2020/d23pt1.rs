use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D23Pt1 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// assert_eq!(Solution::new(&[3,8,9,1,2,5,4,6,7], 10).solve().expect("1"), "92658374");
/// assert_eq!(Solution::new(&[3,8,9,1,2,5,4,6,7].to_string(), 100).solve().expect("1"), "67384529");
/// ```
pub struct Solution
{
	numbers: Vec<usize>,
	count: usize,
}

impl Solution
{
	pub fn new<I: IntoIterator<Item=usize>>(numbers: I, count: usize) -> Self
	{
		Self
		{
			numbers: numbers.into_iter().collect(),
			count,
		}
	}

	pub fn parser<'a>() -> Box<dyn super::super::InputParser<'a>>
	{
		Box::new(Parser)
	}
}

struct Parser;

impl<'a> super::super::InputParser<'a> for Parser
{
	fn year(&self) -> usize
	{
		2020
	}
	fn day(&self) -> usize
	{
		23
	}
	fn part(&self) -> usize
	{
		1
	}
	fn input_url(&self) -> Option<reqwest::Url>
	{
		None
	}
	fn parse(&self, _input: Option<String>, matches: &clap::ArgMatches<'a>) -> Box<dyn super::super::Solution>
	{
		Box::new(Solution::new(
			matches.value_of("input").unwrap().chars().map(|i| i.to_digit(10).unwrap() as usize).collect::<Vec<_>>(),
			matches.value_of("iterations").unwrap().parse().unwrap()
		))
	}
	fn usage<'b>(&self) -> clap::App<'b,'b>
	{
		clap::SubCommand::with_name(&self.name())
			.arg
				( clap::Arg::with_name("input")
				.value_name("INPUT")
				.help("input string as per website")
				.takes_value(true)
				.multiple(false)
				.default_value("186524973")
				)
			.arg
				( clap::Arg::with_name("iterations")
				.value_name("ITERATIONS")
				.short("i")
				.long("iterations")
				.alias("count")
				.help("amount of iterations")
				.allow_hyphen_values(true)
				.takes_value(true)
				.multiple(false)
				.default_value("100")
				)
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		debug!("called with input: {:?}", self.numbers);

		let mut cups = self.numbers.clone();

		let max = cups.iter().max().copied().ok_or(ErrorKind::NoSolution)?;
		let min = cups.iter().min().copied().ok_or(ErrorKind::NoSolution)?;

		debug!("starting {} iterations", self.count);

		debug!("state: {:?}", cups);
		for _ in 0..self.count
		{
			let (&current, rest) = cups.split_first().ok_or(ErrorKind::ParseError)?;
			let (three, rest) = rest.split_at(3);
			debug!("current: {:?}", current);
			debug!("three: {:?}", three);
			debug!("rest: {:?}", rest);
			let insert_after = (min..current).rev().chain((min..=max).rev()).find(|i| !three.contains(i)).ok_or(ErrorKind::NoSolution)?;
			cups = std::iter::once(&insert_after)
				.chain(three.iter())
				.chain(
					rest.iter()
						.chain(std::iter::once(&current))
						.cycle()
						.skip_while(|&&i| i != insert_after)
						.skip(1)
						.take(rest.len())
				)
				.cycle()
				.skip_while(|&&i| i != current)
				.skip(1)
				.take(cups.len())
				.copied()
				.collect();
			debug!("state: {:?}", cups);
		}

		let result = cups.iter()
			.copied()
			.cycle()
			.skip_while(|&i| i != 1)
			.skip(1)
			.take(cups.len() - 1)
			.map(|i| Ok(std::char::from_digit((i%10) as u32,10).ok_or(ErrorKind::ParseError)?))
			.collect::<Result<String>>()?;
		Ok(result)
	}
}

