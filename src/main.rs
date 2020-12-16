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

	let _my_ticket = parts.next().ok_or(ErrorKind::ParseError)?.lines().nth(1).ok_or(ErrorKind::ParseError)?;

	let tickets = parts.next().ok_or(ErrorKind::ParseError)?.lines().skip(1)
		.map(|line| Ok(line.parse::<Ticket>()?))
		.collect::<Result<Vec<Ticket>>>()?;

	let sum: usize = tickets.iter()
		.flat_map(|ticket| ticket.0.iter().copied().filter(|&num| !rules.values().any(|rule| rule.validate(num))))
		.sum();

	println!("{}", sum);
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());
	Ok(())
}

