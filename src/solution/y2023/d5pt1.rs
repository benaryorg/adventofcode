use crate::error::*;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d5pt1::Solution, Solution as S };
/// # env_logger::init();
/// let input = "\
///     seeds: 79 14 55 13\n\
///     \n\
///     seed-to-soil map:\n\
///     50 98 2\n\
///     52 50 48\n\
///     \n\
///     soil-to-fertilizer map:\n\
///     0 15 37\n\
///     37 52 2\n\
///     39 0 15\n\
///     \n\
///     fertilizer-to-water map:\n\
///     49 53 8\n\
///     0 11 42\n\
///     42 0 7\n\
///     57 7 4\n\
///     \n\
///     water-to-light map:\n\
///     88 18 7\n\
///     18 25 70\n\
///     \n\
///     light-to-temperature map:\n\
///     45 77 23\n\
///     81 45 19\n\
///     68 64 13\n\
///     \n\
///     temperature-to-humidity map:\n\
///     0 69 1\n\
///     1 0 69\n\
///     \n\
///     humidity-to-location map:\n\
///     60 56 37\n\
///     56 93 4";
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "35");
/// ```
pub struct Solution
{
	input: String,
}

pub struct Map
{
	name: String,
	// mapped source range, first target value
	map: Vec<(std::ops::Range<usize>, usize)>,
}

impl Map
{
	fn translate(&self, input: usize) -> usize
	{
		trace!("translating {} via {:?}", input, self.name);
		self.map.iter()
			.find(|(range, _)| range.contains(&input))
			.map(|(range, destination)| input - range.start + destination)
			.unwrap_or(input)
	}
}

impl std::str::FromStr for Map
{
	type Err = Error;
	fn from_str(input: &str) -> std::result::Result<Self, Error>
	{
		trace!("parsing map {}", input);
		let (name, input) = input.split_once(' ').ok_or(Error::AocParsing)?;
		let map = input.lines()
			.skip(1)
			.map(|line|
			{
				let mut parts = line.split_whitespace();
				let destination: usize = parts.next().ok_or(Error::AocParsing)?.parse()?;
				let start: usize = parts.next().ok_or(Error::AocParsing)?.parse()?;
				let length: usize = parts.next().ok_or(Error::AocParsing)?.parse()?;
				Ok((start..(start + length), destination))
			})
			.collect::<std::result::Result<_, Self::Err>>()?;

		Ok(Map
		{
			name: name.to_string(),
			map,
		})
	}
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

		let (seeds, input) = self.input.split_once("\n\n").ok_or(Error::AocParsing)?;
		let seeds = seeds.strip_prefix("seeds: ")
			.ok_or(Error::AocParsing)?
			.split_whitespace()
			.map(|s| -> Result<usize>
			{
				s.parse().context(Error::AocParsing)
			})
			.collect::<Result<Vec<usize>>>()?;

		debug!("seeds: {:?}", seeds);

		let maps = input.split("\n\n")
			.map(|s|
			{
				s.parse().context(Error::AocParsing)
			})
			.collect::<Result<Vec<Map>>>()?;

		let translated = seeds.into_iter()
			.map(|seed|
			{
				maps.iter()
					.fold(seed, |seed, map| map.translate(seed))
			})
			.collect::<Vec<_>>();

		Ok(format!("{}", translated.iter().min().unwrap()))
	}
}

