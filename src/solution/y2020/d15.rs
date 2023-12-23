use crate::error::*;

/// # Examples
///
/// With 2020 iterations:
///
/// ```
/// # use adventofcode::solution::
/// # {
/// #     y2020::D15 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// assert_eq!(Solution::new("0,3,6".to_string(), 2020).solve().expect("1.1"), "436");
/// assert_eq!(Solution::new("1,3,2".to_string(), 2020).solve().expect("1.2"), "1");
/// assert_eq!(Solution::new("2,1,3".to_string(), 2020).solve().expect("1.3"), "10");
/// assert_eq!(Solution::new("1,2,3".to_string(), 2020).solve().expect("1.4"), "27");
/// assert_eq!(Solution::new("2,3,1".to_string(), 2020).solve().expect("1.5"), "78");
/// assert_eq!(Solution::new("3,2,1".to_string(), 2020).solve().expect("1.6"), "438");
/// assert_eq!(Solution::new("3,1,2".to_string(), 2020).solve().expect("1.7"), "1836");
/// ```
///
/// With 30000000 iterations:
///
/// ```no_run
/// # use adventofcode::solution::
/// # {
/// #     y2020::D15 as Solution,
/// #     Solution as S,
/// # };
/// # env_logger::init();
/// assert_eq!(Solution::new("0,3,6".to_string(), 30000000).solve().expect("2.1"), "175594");
/// ```
pub struct Solution
{
	input: String,
	iterations: usize,
}

impl Solution
{
	pub fn new(input: String, iterations: usize) -> Self
	{
		Self { input, iterations, }
	}

	pub fn parser_pt1<'a>() -> Box<dyn super::super::InputParser<'a>>
	{
		Box::new(Parser
		{
			iterations: "2020",
			part: 1,
		})
	}

	pub fn parser_pt2<'a>() -> Box<dyn super::super::InputParser<'a>>
	{
		Box::new(Parser
		{
			iterations: "30000000",
			part: 2,
		})
	}
}

struct Parser
{
	iterations: &'static str,
	part: usize,
}

impl<'a> super::super::InputParser<'a> for Parser
{
	fn year(&self) -> usize
	{
		2020
	}
	fn day(&self) -> usize
	{
		15
	}
	fn part(&self) -> usize
	{
		self.part
	}
	fn input_url(&self) -> Option<reqwest::Url>
	{
		None
	}
	fn parse(&self, _input: Option<String>, matches: &clap::ArgMatches) -> Box<dyn super::super::Solution>
	{
		Box::new(Solution::new(
			matches.get_one::<String>("input").unwrap().to_string(),
			*matches.get_one("iterations").unwrap(),
		))
	}
	fn usage<'b>(&self) -> clap::Command
	{
		clap::Command::new(self.name())
			.arg
				( clap::Arg::new("input")
				.value_name("INPUT")
				.help("input string as per website")
				.default_value("9,19,1,6,0,5,4")
				)
			.arg
				( clap::Arg::new("iterations")
				.value_name("ITERATIONS")
				.short('i')
				.long("iterations")
				.alias("count")
				.help("amount of iterations")
				.allow_hyphen_values(true)
				.default_value(self.iterations)
				)
	}
}

impl super::super::Solution for Solution
{
	fn solve(&self) -> Result<String>
	{
		let nums = self.input.split(',')
			.map(|num| Ok(num.parse::<usize>()?))
			.collect::<Result<Vec<_>>>()?;

		let mut map = nums.iter()
			.copied()
			.enumerate()
			.map(|(idx,num)| (num,idx+1))
			.take(nums.len() - 1)
			.collect::<std::collections::BTreeMap<_,_>>();

		let next = *nums.last().ok_or(Error::AocNoSolution)?;
		let mut turn = nums.len();

		let result = std::iter::successors(Some(next),|&last|
		{
			let mut next = 0;
			map.entry(last)
				.and_modify(|atime|
				{
					next = turn - *atime;
					*atime = turn;
				})
				.or_insert(turn);
			turn += 1;

			Some(next)
		})
			.take(self.iterations + 1 - nums.len())
			.last()
			.ok_or(Error::AocNoSolution)?;

		Ok(format!("{}", result))
	}
}

