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

struct Mask
{
	zero: u64,
	one: u64,
}

impl Mask
{
	fn apply(&self, num: u64) -> u64
	{
		(num | self.one) & !self.zero
	}
}

impl std::str::FromStr for Mask
{
	type Err = Error;
	fn from_str(input: &str) -> Result<Self>
	{
		let mut zero = 0;
		let mut one = 0;

		for ch in input.chars()
		{
			zero <<= 1;
			one <<= 1;
			match ch
			{
				'1' => one |= 1,
				'0' => zero |= 1,
				'X' => {},
				_ => bail!(ErrorKind::ParseError),
			}
		}

		Ok(Mask { zero, one, })
	}
}

use error::*;

fn main() -> Result<()>
{
	let timer = std::time::Instant::now();

	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/14/input").send()?.text()?;

	println!("fetched in {:.3}s", timer.elapsed().as_secs_f64());
	let timer = std::time::Instant::now();

	let result = body.lines()
		.fold((Default::default(),"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse::<Mask>()?),|(mut acc,mask): (std::collections::BTreeMap<usize,u64>,Mask), next|
		{
			if next.starts_with("mask =")
			{
				(acc, next.split("=").nth(1).ok_or(ErrorKind::ParseError).unwrap().trim().parse().unwrap())
			}
			else
			{
				lazy_static!
				{
					static ref RE: Regex = Regex::new(r"\Amem\[(?P<key>\d+)\] = (?P<value>\d+)\z").unwrap();
				}
				let captures = RE.captures(next).ok_or(ErrorKind::ParseError).unwrap();
				acc.insert(captures.name("key").unwrap().as_str().parse().unwrap(),mask.apply(captures.name("value").unwrap().as_str().parse().unwrap()));
				(acc,mask)
			}
		}).0;

	println!("{:?}", result.values().sum::<u64>());
	println!("done in {:.3}s", timer.elapsed().as_secs_f64());

	Ok(())
}

