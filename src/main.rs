#[macro_use]
extern crate error_chain;
extern crate reqwest;

use std::collections::BTreeSet;

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
		}
	}
}

use error::*;

fn main() -> Result<()>
{
	let headers: reqwest::header::HeaderMap = [(reqwest::header::COOKIE,format!("session={}",std::env!("ADVENTOFCODE_SESSION")).parse().unwrap())].iter().cloned().collect();
	let http = reqwest::blocking::Client::builder().default_headers(headers).build()?;
	let body = http.get("https://adventofcode.com/2020/day/1/input").send()?.text()?;
	let numbers = body.lines().map(|s| Ok(s.parse::<usize>()?)).collect::<Result<Vec<_>>>()?;
	let target = 2020;
	let (low,high): (BTreeSet<_>,BTreeSet<_>) = numbers.into_iter().partition(|&i| i < target/2);
	for &low in std::iter::once(&0).chain(low.iter())
	{
		let result = high.iter()
			.take_while(|&high| high + low <= target)
			.find(|&high| high + low == target);

		if let Some(result) = result
		{
			println!("{}", low.max(1) * result);
			return Ok(());
		}
	}

	bail!(ErrorKind::NoSolution);
}

