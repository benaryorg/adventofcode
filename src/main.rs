#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate num;
extern crate regex;
extern crate reqwest;

#[allow(unused_imports)]
use regex::Regex;

pub mod error
{
	error_chain!
	{
		links
		{
		}

		foreign_links
		{
			Io(::std::io::Error);
			Parse(::std::string::ParseError);
			ParseInt(::std::num::ParseIntError);
			Reqwest(::reqwest::Error);
		}

		errors
		{
			NoSolution {}
			ParseError {}
		}
	}
}

use error::*;

#[derive(Clone,Debug)]
struct Rule
{
	ranges: Vec<std::ops::RangeInclusive<usize>>,
}

impl Rule
{
	fn validate(&self, number: usize) -> bool
	{
		self.ranges.iter().any(|range| range.contains(&number))
	}
}

impl std::str::FromStr for Rule
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let ranges = input.split(" or ")
			.map(|string|
			{
				string.split('-')
					.map(|i| Ok(i.parse()?))
					.collect::<Result<Vec<usize>>>()
					.and_then(|vec|
					{
						if vec.len() != 2
						{
							bail!(ErrorKind::ParseError);
						}
						Ok(vec[0]..=vec[1])
					})
			})
			.collect::<Result<_>>()?;
		Ok(Self { ranges, })
	}
}

struct Ticket(Vec<usize>);

impl std::str::FromStr for Ticket
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		Ok(Ticket(input.split(',').map(|i| Ok(i.parse()?)).collect::<Result<_>>()?))
	}
}

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/16/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let mut parts = body.split("\n\n");

	let rules = parts.next().ok_or(ErrorKind::ParseError)?.lines()
		.map(|line|
		{
			let mut split = line.splitn(2,": ");
			let name = split.next().ok_or(ErrorKind::ParseError)?;
			let rule = split.next().ok_or(ErrorKind::ParseError)?.parse::<Rule>()?;
			Ok((name,rule))
		})
		.collect::<Result<std::collections::HashMap<_,_>>>()?;

	let my_ticket = parts.next().ok_or(ErrorKind::ParseError)?.lines().nth(1).ok_or(ErrorKind::ParseError)?.parse::<Ticket>()?;

	let tickets = parts.next().ok_or(ErrorKind::ParseError)?.lines().skip(1)
		.map(|line| Ok(line.parse::<Ticket>()?))
		.collect::<Result<Vec<Ticket>>>()?
		.into_iter()
		.filter(|ticket| ticket.0.iter().all(|&num| rules.values().any(|rule| rule.validate(num))))
		.collect::<Vec<_>>();

	let mut possibilities = rules.iter()
		.flat_map(|kv|
		{
			(0..rules.len())
				.map(
				{
					let kv = kv.clone();
					move |i| (i,kv.clone())
				})
		})
		.filter(|&(idx,(_,rule))| tickets.iter().all(|ticket| ticket.0.get(idx).map(|&value| rule.validate(value)).unwrap_or(false)))
		.map(|(idx,(name,_))| (name,idx))
		.collect::<Vec<_>>();

	let mut mapping = std::collections::HashMap::new();

	loop
	{
		if possibilities.is_empty()
		{
			break;
		}
		let map = possibilities.iter().map(|(_,idx)| idx).copied()
			.fold(std::collections::BTreeMap::<usize,usize>::new(), |mut map, value|
			{
				map.entry(value)
					.and_modify(|value| *value += 1)
					.or_insert(1);
				map
			});
		for (key,_) in map.into_iter().filter(|&(_,value)| value == 1)
		{
			let (name,_) = possibilities.iter().cloned().find(|(_,idx)| *idx == key).unwrap();
			mapping.insert(name, key);
			possibilities.retain(|e| e.1 != key && e.0 != name);

			println!("{}: {}", name, key);
		}
	}

	println!("{}", mapping.into_iter().filter(|(name,_)| name.starts_with("departure")).map(|(_,idx)| Ok(my_ticket.0.get(idx).ok_or(ErrorKind::ParseError)?)).collect::<Result<Vec<_>>>()?.into_iter().product::<usize>());
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	Ok(())
}

