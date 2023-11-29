use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D23Pt2 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// assert_eq!(Solution::new([3,8,9,1,2,5,4,6,7].iter().copied(), 10_000_000).solve().expect("1"), "149245887792");
/// ```
pub struct Solution
{
	cups: Box<[usize;NUM_CUPS]>,
	first: usize,
	iterations: usize,
}

const NUM_CUPS: usize = 1_000_000;

impl Solution
{
	pub fn new<I: IntoIterator<Item=usize>>(numbers: I, iterations: usize) -> Self
	{
		let mut cups = Box::new([0;NUM_CUPS]);
		let len = cups.len();
		for (idx, cup) in cups.iter_mut().enumerate()
		{
			*cup = (idx+1)%len;
		}
		let numbers = numbers.into_iter().collect::<Vec<_>>();
		for (from,to) in numbers.windows(2).map(|s| (s[0]-1,s[1]-1))
			.chain(std::iter::once((numbers.last().copied().unwrap()-1,numbers.len())))
			.chain(std::iter::once((cups.len()-1,numbers.first().copied().unwrap()-1)))
		{
			debug!("cups[{}] = {};",from,to);
			cups[from] = to;
		}
		Self { cups, iterations, first: *numbers.first().unwrap_or(&1)-1, }
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
		2
	}
	fn input_url(&self) -> Option<reqwest::Url>
	{
		None
	}
	fn parse(&self, _input: Option<String>, matches: &clap::ArgMatches) -> Box<dyn super::super::Solution>
	{
		let provided_numbers = matches.get_one::<String>("input").unwrap().chars().map(|i| i.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
		Box::new(Solution::new(
			provided_numbers,
			*matches.get_one("iterations").unwrap()
		))
	}
	fn usage<'b>(&self) -> clap::Command
	{
		clap::Command::new(self.name())
			.arg
				( clap::Arg::new("input")
				.value_name("INPUT")
				.help("input string as per website")
				.default_value("186524973")
				)
			.arg
				( clap::Arg::new("iterations")
				.value_name("ITERATIONS")
				.short('i')
				.long("iterations")
				.alias("count")
				.help("amount of iterations")
				.allow_hyphen_values(true)
				.default_value("10000000")
				)
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let mut cups = self.cups.clone();
		let mut ptr = self.first;

		debug!("sum: {}", cups.iter().sum::<usize>());
		debug!("first: {}", ptr);
		debug!("cups: {:?}...", &cups.iter().take(16).collect::<Vec<_>>());
		debug!("{:?}", std::iter::successors(Some(3), |n| Some(cups.get(n-1).unwrap()%cups.len()+1)).take(16.min(cups.len())).collect::<Vec<_>>());

		debug!("starting {} iterations", self.iterations);
		for _ in 0..self.iterations
		{
			let current = cups[ptr];
			let first = cups[current];
			let second = cups[first];
			let third = cups[second];
			let destination = (0..ptr).rev().chain((ptr..cups.len()).rev()).find(|i| ![current,first,second].contains(i)).unwrap();
			trace!("ptr: {}", ptr);
			trace!("current: {}", current);
			trace!("first: {}", first);
			trace!("second: {}", second);
			trace!("third: {}", third);
			trace!("destination: {}", destination);
			cups.swap(ptr,destination);
			cups.swap(ptr,second);
			ptr = third;
			debug!("{:?}", std::iter::successors(Some(3), |n| Some(cups.get(n-1).unwrap()%cups.len()+1)).take(16.min(cups.len())).collect::<Vec<_>>());
		}
		debug!("cups: {:?}...", &cups.iter().take(16).collect::<Vec<_>>());

		let first = cups[0]+1;
		let second = cups[cups[0]]+1;
		let result = first * second;
		debug!("{}*{}={}", first, second, result);

		Ok(format!("{}", result))
	}
}

