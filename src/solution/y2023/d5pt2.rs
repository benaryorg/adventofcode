use crate::error::*;

use std::ops::Range;

/// # Examples
///
/// ```
/// # use adventofcode::solution::{ y2023::d5pt2::Solution, Solution as S };
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
/// assert_eq!(Solution::new(input.to_string()).solve().unwrap(), "46");
/// ```
pub struct Solution
{
	input: String,
}

type Mapping = (Range<usize>, Range<usize>);

#[derive(Clone, Debug)]
pub struct Map
{
	name: String,
	// mapped source range, first target value
	map: Vec<Mapping>,
}

impl Map
{
	fn translate(&self, input: usize) -> usize
	{
		let result = self.map.iter()
			.find(|(range, _)| range.contains(&input))
			.map(|(range, destination)| input - range.start + destination.start)
			.unwrap_or(input);
		trace!("{} translate {} to {}", self.name, input, result);
		result
	}

	fn rtranslate(&self, input: usize) -> usize
	{
		let result = self.map.iter()
			.find(|(_, destination)| destination.contains(&input))
			.map(|(range, destination)| input - destination.start + range.start)
			.unwrap_or(input);
		trace!("{} rtranslate {} to {}", self.name, input, result);
		result
	}

	fn merge(&self, other: &Map) -> Map
	{
		let name = format!("{}-to-{}", self.name.split_once('-').unwrap().0, other.name.rsplit_once('-').unwrap().1);
		debug!("merging {} and {} to {}", self.name, other.name, name);
		trace!("merge:\n{:#?}\n{:#?}", self, other);

		let points = other.map.iter()
			.flat_map(|(range, _)| [range.start, range.end, self.rtranslate(range.start), self.rtranslate(range.end)])
			.chain(self.map.iter()
				.flat_map(|(range, _)| [range.start, range.end])
			)
			.collect::<std::collections::BTreeSet<_>>();

		trace!("{} merge points: {:?}", name, points);

		let map = points.into_iter().collect::<Vec<_>>().windows(2)
			.map(|slice| slice[0]..slice[1])
			.map(|range|
			{
				let start = other.translate(self.translate(range.start));
				let end = other.translate(self.translate(range.end - 1));
				(range, start..(end + 1))
			})
			.collect();

		Map
		{
			name,
			map,
		}
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
				Ok((start..(start + length), destination..(destination + length)))
			})
			.collect::<std::result::Result<Vec<_>, Self::Err>>()?;

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

		let seeds = seeds.chunks_exact(2)
			.map(|slice|
			{
				slice[0]..(slice[0] + slice[1])
			})
			.collect::<Vec<_>>();

		let map = maps.iter()
			.skip(1)
			.fold(maps.first().unwrap().clone(), |a: Map, b| a.merge(b));

		debug!("maps compiled");

		for i in 0..100
		{
			let stacked = maps.iter().fold(i, |seed, map| map.translate(seed));
			let merged = map.translate(i);
			debug!("{} {}: should {} is {}", i, stacked == merged, stacked, merged);
		}

		let mut map_entries = map.map.iter()
			.collect::<Vec<_>>();
		map_entries.sort_by_key(|(_, dest)| dest.start);

		let result = map_entries.into_iter()
			.flat_map(|(range, _)|
			{
				seeds.iter()
					.filter(move |seed|
					{
						range.contains(&seed.start) || range.contains(&(seed.end - 1)) || seed.contains(&range.start) || seed.contains(&(range.end - 1))
					})
					.flat_map(move |seed| range.start.max(seed.start)..range.end.min(seed.end))
			})
			.next().unwrap();

		Ok(format!("{}", map.translate(result)))
	}
}

