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
	let numbers = body.lines().map(|s| Ok(s.parse::<usize>()?)).collect::<Result<BTreeSet<_>>>()?;
	let target = 2020;
	for &base in numbers.iter()
	{
		let target = target - base;
		for &low in numbers.range(..(target/2))
		{
			let target = target - low;
			if let Some(&high) = numbers.get(&target)
			{
				println!("{}", base * low * high);
				return Ok(());
			}
		}
	}

	bail!(ErrorKind::NoSolution);
}

